use elastic::prelude::*;
use std::error::Error;
use super::event_structs::Event;
use serde_json::Value;

pub fn get_example_object() -> Result<Vec<Value>, Box<Error>> {
    println!("test!");
    // A reqwest HTTP client and default parameters.
    // The builder includes the base node url (http://localhost:9200).
    let client = SyncClientBuilder::new()
        .base_url("http://hammer.caida.org:9200")
        .build()?;

    // A search request with a freeform body.
    let res = client
        .search::<Value>()
        //.search::<Value>()
        .index("_all")
        .body(json!({
            "query": {
                "match":{
                    "_id": "defcon-1539505500-4739"
                }
            }
        }))
        .send()?;

    // Iterate through the hits in the response.
    let mut res_vec:Vec<Value> = Vec::new();
    for hit in res.hits() {
        res_vec.push(hit.document().unwrap().clone());
    }

    Ok(res_vec)
}

// pub fn get_example_object() -> Result<Vec<Event>, Box<Error>> {
//     println!("test!");
//     // A reqwest HTTP client and default parameters.
//     // The builder includes the base node url (http://localhost:9200).
//     let client = SyncClientBuilder::new()
//         .base_url("http://hammer.caida.org:9200")
//         .build()?;

//     // A search request with a freeform body.
//     let res = client
//         .search::<Event>()
//         //.search::<Value>()
//         .index("_all")
//         .body(json!({
//             "query": {
//                 "match":{
//                     "_id": "defcon-1539505500-4739"
//                 }
//             }
//         }))
//         .send()?;

//     // Iterate through the hits in the response.
//     let mut resVec:Vec<Event> = Vec::new();
//     for hit in res.hits() {
//         resVec.push(hit.document().unwrap().clone());
//     }

//     Ok(resVec)
// }
