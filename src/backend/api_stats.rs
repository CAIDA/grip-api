// if query_type == "total"{
// let stats: Value = reqwest::get("http://clayface.caida.org:9200/_cat/indices?format=json").unwrap().json().unwrap();
// return Json(json!(stats))
use serde_json::json;
use serde_json::Value;
use rocket_contrib::json::Json;
use std::str::FromStr;

#[get("/json/stats/today")]
pub fn json_stats_today() -> Json<Value> {
    let count_query: Value = json!(
        {
            "query": {
                "bool": {
                    "must": {
                        "term": {
                            "inference.tr_worthy": true
                        }
                    },
                    "must_not": {
                        "match": {
                            "position.keyword": "FINISHED"
                        }
                    },
                    "filter": {
                        "range": {
                            "view_ts": {
                                "gte": "now-1d/d",
                                "lt": "now/d"
                            }
                        }
                    }
                }
            }
        }
    );

    let client = reqwest::Client::new();

    let mut return_value = json!({});

    let event_types = vec!("moas", "submoas", "defcon", "edges", "*");
    for event_type in event_types {
        let res: Value = client
            .post(format!("http://clayface.caida.org:9200/hijacks-{}/_count", event_type).as_str())
            .json(&count_query).send().unwrap().json().unwrap();
        let mut t = event_type;
        if event_type == "*" {
            t = "all";
        }
        return_value[t] = json!({"count": res["count"]});
    }
    return Json(return_value);
}

#[get("/json/stats/total")]
pub fn json_stats_total() -> Json<Value> {
    let count_query: Value = json!(
        {
            "query": {
                "bool": {
                    "must": {
                        "term": {
                            "inference.tr_worthy": true
                        }
                    },
                    "must_not": {
                        "match": {
                            "position.keyword": "FINISHED"
                        }
                    }
                }
            }
        }
    );

    let client = reqwest::Client::new();
    let mut return_value = json!({});

    // get documents count
    let event_types = vec!("moas", "submoas", "defcon", "edges", "*");
    for event_type in event_types {
        let res: Value = client
            .post(format!("http://clayface.caida.org:9200/hijacks-{}/_count", event_type).as_str())
            .json(&count_query).send().unwrap().json().unwrap();
        let mut t = event_type;
        if event_type == "*" {
            t = "all";
        }
        return_value[t] = json!({"count": res["count"]});
    }

    // get storage
    let stats: Value = reqwest::get("http://clayface.caida.org:9200/_cat/indices?bytes=mb&format=json").unwrap().json().unwrap();
    let mut total_size = 0;
    for record in stats.as_array().unwrap() {
        let index_name = String::from_str(record["index"].as_str().unwrap()).unwrap();
        let store_size: i32 = record["store.size"].as_str().unwrap().to_string().parse().unwrap();
        total_size += store_size;
        if !index_name.contains("hijacks") {
            // not the index we care about
            continue;
        }
        let event_type = index_name.split("-").collect::<Vec<&str>>()[1];
        return_value[event_type]["store.size.mb"] =  json!(store_size);
    }
    return_value["all"]["store.size.mb"] =  json!(total_size);
    return Json(return_value);
}
