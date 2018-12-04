use std::error::Error;

use elastic::prelude::*;
use serde_json::Value;

use backend::errors::MyError;

pub struct ElasticSearchBackend {
    es_client: SyncClient,
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

    pub fn get_event_by_id(&self, id: &str) -> Result<Value, Box<Error>> {
        let res = self.es_client
            .search::<Value>()
            .index("hijacks*")
            .body(json!({
            "size":1,
            "query": {
                "match" : {
                    "id.keyword" : id,
                }
            }
        }))
            .send()?;

        for hit in res.hits() {
            return Ok(hit.document().unwrap().clone())
        }
        Err(Box::new(MyError("Oops".into())))
    }

    pub fn list_all_events(&self) -> Result<Vec<Value>, Box<Error>> {
        let res = self.es_client
            .search::<Value>()
            .index("hijacks*")
            .body(json!({
            "from":0, "size":20,
            "query": {
                "match_all": {}
            },
            "sort": { "view_ts": { "order": "desc" }}
        }))
            .send()?;

        // Iterate through the hits in the response and build a vector.
        let mut res_vec: Vec<Value> = Vec::new();
        for hit in res.hits() {
            res_vec.push(hit.document().unwrap().clone());
        }

        Ok(res_vec)
    }

}

