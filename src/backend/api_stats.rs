// if query_type == "total"{
// let stats: Value = reqwest::get("http://clayface.caida.org:9200/_cat/indices?format=json").unwrap().json().unwrap();
// return Json(json!(stats))
use serde_json::json;
use serde_json::Value;
use rocket_contrib::json::Json;
use std::str::FromStr;
use rocket::http::RawStr;

fn get_type_count(event_type: &str, only_today: bool) -> Value{
    let client = reqwest::Client::new();

    let mut range_filter = json!({"view_ts":{}});
    if only_today{
        range_filter["view_ts"]["gte"] = json!("now-1d");
        range_filter["view_ts"]["lt"] = json!("now");
    }

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
                        "range": range_filter
                    }
                }
            }
        }
    );

    let etype = match event_type{
        "all" => "*",
        _ => event_type,
    };

    return client.post(format!("http://clayface.caida.org:9200/hijacks-{}/_count", etype).as_str())
        .json(&count_query).send().unwrap().json::<Value>().unwrap();
}

fn get_total_stats() -> Value{
    let mut return_value = json!({"moas":json!({}), "submoas":json!({}), "defcon":json!({}), "edges":json!({}), "all":json!({}), });
    let stats: Value = reqwest::get("http://clayface.caida.org:9200/_cat/indices?bytes=b&format=json").unwrap().json().unwrap();
    let mut total_size = 0;
    for record in stats.as_array().unwrap() {
        let index_name = String::from_str(record["index"].as_str().unwrap()).unwrap();
        let store_size: i64 = record["pri.store.size"].as_str().unwrap().to_string().parse().unwrap();
        total_size += store_size;
        if !index_name.contains("hijacks") {
            // not the index we care about
            continue;
        }
        let event_type = index_name.split("-").collect::<Vec<&str>>()[1];
        return_value[event_type]["bytes"] =  json!(store_size);
    }
    return_value["all"]["bytes"] =  json!(total_size);
    return return_value
}

#[get("/json/stats/<event_type>", rank=2)]
pub fn json_stats_by_type(event_type: &RawStr) -> Json<Value> {
    let type_count_today = get_type_count(event_type, true);
    let type_count_total = get_type_count(event_type, false);
    let total_stats = get_total_stats();

    Json(json!({
    "total":{
        "count": type_count_total["count"],
        "bytes": total_stats[event_type.as_str()]["bytes"]
    },
    "today":{
        "count": type_count_today["count"],
    },
    }))
}

#[get("/json/stats/today")]
pub fn json_stats_today() -> Json<Value> {
    let mut return_value = json!({});

    let event_types = vec!("moas", "submoas", "defcon", "edges", "*");
    for event_type in event_types {
        let res: Value = get_type_count(event_type, true);
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
    let mut return_value = json!({});

    // get documents count
    let event_types = vec!("moas", "submoas", "defcon", "edges", "*");
    for event_type in event_types {
        let res: Value = get_type_count(event_type, false);
        let mut t = event_type;
        if event_type == "*" {
            t = "all";
        }
        return_value[t] = json!({"count": res["count"]});
    }

    // get storage
    let stats = get_total_stats();
    let event_types = vec!("moas", "submoas", "defcon", "edges", "all");
    for event_type in event_types {
        return_value[event_type]["bytes"] = stats[event_type]["bytes"].clone();
    }
    return Json(return_value);
}
