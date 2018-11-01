use std::error::Error;
use serde_json::Value;
use elastic::prelude::*;

pub fn test() -> Result<(), Box<Error>> {
    println!("test!");
    // A reqwest HTTP client and default parameters.
    // The builder includes the base node url (http://localhost:9200).
    let client = SyncClientBuilder::new()
        .base_url("http://hammer.caida.org:9200")
        .build()?;

    // A search request with a freeform body.
    let res = client.search::<Value>()
        .index("_all")
        .body(json!({
            "query": {
                "match_all": {}
            }
        }))
        .send()?;

    // Iterate through the hits in the response.
    for hit in res.hits() {
        println!("{:?}", hit);
    }

    Ok(())
}
