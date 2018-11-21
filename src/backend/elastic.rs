use elastic::prelude::*;
use std::error::Error;
use serde_json::Value;
use std::fmt;

#[derive(Debug)]
struct MyError(String);
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "There is an error: {}", self.0)
    }
}
impl Error for MyError {}

pub fn get_event_by_id(id: &str) ->Result<Value, Box<Error>> {

    println!("test!");
    let client = SyncClientBuilder::new()
        .base_url("http://hammer.caida.org:9200")
        .build()?;

    // A search request with a freeform body.
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