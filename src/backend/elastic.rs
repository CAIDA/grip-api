use elastic::prelude::*;
use std::error::Error;
use serde_json::Value;

pub fn list_all_events() -> Result<Vec<Value>, Box<Error>> {
    println!("test!");
    let client = SyncClientBuilder::new()
        .base_url("http://hammer.caida.org:9200")
        .build()?;

    // A search request with a freeform body.
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