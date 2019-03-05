use std::error::Error;

use elastic::prelude::*;
use serde_json::json;
use serde_json::Value;

use crate::backend::errors::MyError;

pub struct ElasticSearchBackend {
    es_client: SyncClient,
}

pub struct SearchResult {
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
        let event_type: &str = id.split("-").collect::<Vec<&str>>()[0];
        let doc: Value = reqwest::get(format!("http://clayface.caida.org:9200/hijacks-{}/event_result/{}",event_type, id).as_str())
            .unwrap().json().unwrap();
        if doc["found"] == true {
            return Ok(SearchResult { results: vec!(doc["_source"].clone()), total: 1 });
        } else {
            Err(Box::new(MyError("Oops".into())))
        }
    }

    pub fn list_events(&self, event_type: &str, start: &Option<usize>, max: &Option<usize>,
                       asn: &Option<usize>, prefix: &Option<String>,
                       ts_start: &Option<String>, ts_end: &Option<String>, benign: &Option<bool>,
                       tags: &Option<String>)
                       -> Result<SearchResult, Box<Error>> {
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

        // match must terms
        let mut must_terms = vec!();
        match benign {
            Some(b) => if *b {
                must_terms.push(json!({ "term": { "inference.tr_worthy" : false }}));
            } else {
                must_terms.push(json!({ "term": { "inference.tr_worthy" : true }}));
            },
            None => must_terms.push(json!({ "term": { "inference.tr_worthy" : true }})),
        }
        match prefix {
            Some(p) => {
                // https://stackoverflow.com/questions/40573981/multiple-should-queries-with-must-query
                let mut pfx_must = vec!();
                pfx_must.push(json!({ "prefix": { "pfx_events.sub_pfx.keyword" : p }}));
                pfx_must.push(json!({ "prefix": { "pfx_events.super_pfx.keyword" : p }}));
                pfx_must.push(json!({ "prefix": { "pfx_events.prefix.keyword" : p }}));
                must_terms.push(json!({"bool": {"minimum_should_match": 1, "should": pfx_must}}));
            }
            _ => {}
        }
        match asn {
            Some(value) => {
                // https://stackoverflow.com/questions/40573981/multiple-should-queries-with-must-query
                let mut asn_must = vec!();
                asn_must.push(json!({ "match": { "pfx_events.origins" : value }}));
                asn_must.push(json!({ "match": { "pfx_events.as1.keyword" : value }}));
                asn_must.push(json!({ "match": { "pfx_events.as2.keyword" : value }}));
                must_terms.push(json!({"bool": {"minimum_should_match": 1, "should": asn_must}}));
            }
            _ => {}
        }
        match tags {
            Some(tags_string) => {
                let tags_lst = tags_string.split(",").collect::<Vec<&str>>();
                for t in tags_lst{
                    must_terms.push(json!({"exists":{"field":format!("tags.{}", t)}}))
                }
            }
            _ => {}
        }

        let query: serde_json::Value = json!({
                "from":start, "size":max_entries,
                "query": {
                    "bool": {
                        "must": must_terms,
                        "must_not": { "match": { "position.keyword": "FINISHED" } },
                        "filter": {
                            "range": range_filter
                        }
                    }
                },
                "sort": { "view_ts": { "order": "desc" }}
            });

        println!("{:#?}", query);

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

        Ok(SearchResult { results: res_vec, total: res.total() })
    }
}
