use std::error::Error;

use crate::backend::errors::MyError;
use elastic::prelude::*;
use serde_json::json;
use serde_json::Value;

pub struct ElasticSearchBackend {
    es_client: SyncClient,
}

pub struct SearchResult{
    pub results: Vec<Value>,
    pub total: u64,
}

impl ElasticSearchBackend {
    // constructor static method
    pub fn new(base_url: &str) -> Result<ElasticSearchBackend, Box<Error>> {
        let client = SyncClientBuilder::new()
            // .base_url("http://hammer.caida.org:9200")
            .base_url(base_url)
            .build()?;
        Ok(ElasticSearchBackend { es_client: client })
    }

    pub fn get_event_by_id(&self, id: &str) -> Result<SearchResult, Box<Error>> {
        let res = self
            .es_client
            .search::<Value>()
            .index("hijacks*")
            .body(json!({
                "size":1,
                "query": {
                    "bool": {
                        "must": { "match": { "id.keyword" : id }},
                        "must_not": { "match": { "position.keyword": "FINISHED"  }},
                    }
                }
            }))
            .send()?;

        for hit in res.hits() {
            let doc = hit.document().unwrap().clone();

            return Ok( SearchResult{results: vec!(doc), total: res.total()} );
        }
        Err(Box::new(MyError("Oops".into())))
    }

    pub fn list_events(&self, event_type: &str, start: &Option<usize>, max: &Option<usize>,
                       ts_start: &Option<String>, ts_end: &Option<String>)-> Result<SearchResult, Box<Error>> {
        let mut etype = event_type.to_owned();
        if etype == "all" {
            etype = "*".to_owned();
        }

        let mut range_filter = json!({"view_ts":{}});
        match ts_start {
            Some(start_str) => range_filter["view_ts"]["gte"] = json!(start_str),
            _ => {}
        };
        match ts_end {
            Some(end_str) => range_filter["view_ts"]["lte"] = json!(end_str),
            _ => {}
        };

        let max_entries = match max {
            Some(n) => n.to_owned() as i32,
            None => 100 as i32
        };

        let query = json!({
                "from":start, "size":max_entries,
                "query": {
                    "bool": {
                        "must": { "term": { "inference.tr_worthy" : true }},
                        "must_not": { "match": { "position.keyword": "FINISHED" } },
                        "filter": {
                            "range": range_filter
                        }
                    }
                },
                "sort": { "view_ts": { "order": "desc" }}
            });



        let res = self
            .es_client
            .search::<Value>()
            .index(format!("hijacks-{}", etype))
            .body(query)
            .send()?;


        // Iterate through the hits in the response and build a vector.
        let mut res_vec: Vec<Value> = Vec::new();
        for hit in res.hits() {
            let mut doc = json!(hit.document().unwrap());
            doc["_esid"] = json!(hit.index().to_owned());
            res_vec.push(doc);
        }

        Ok(SearchResult{results: res_vec, total: res.total()})
    }
}
