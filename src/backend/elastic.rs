use elastic::prelude::*;
use serde_json::Value;
use std::error::Error;
use backend::errors::MyError;

struct ElasticSearchBackend {

}



pub fn get_event_by_id(id: &str) ->Result<Value, Box<Error>> {

    let client = SyncClientBuilder::new()
        .base_url("http://clayface.caida.org:9200")
        .build()?;

    let res = client
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

pub fn list_all_events() -> Result<Vec<Value>, Box<Error>> {
    let client = SyncClientBuilder::new()
        .base_url("http://clayface.caida.org:9200")
        .build()?;

    let res = client
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
    let mut res_vec:Vec<Value> = Vec::new();
    for hit in res.hits() {
        res_vec.push(hit.document().unwrap().clone());
    }

    Ok(res_vec)
}