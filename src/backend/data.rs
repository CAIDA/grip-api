use serde_json::{json,Value};
use crate::backend::elastic::SearchResult;
use std::collections::HashSet;

/// shared state across rocket threads
pub struct SharedData {
    pub es_url: String,
    pub resource_dir: String,
}

/// process raw event data from elastic-search and produce a filtered verison
/// with only data needed for front-end
pub fn filter_event_list(query_result: &SearchResult) -> Vec<Value> {
    let mut res_vec: Vec<Value> = Vec::new();
    for value in &query_result.results {
        let mut event = json!({});
        // filter easy fields
        for field in vec!["event_type", "view_ts", "finished_ts", "duration", "external", "id"] {
            event[field] = value[field].to_owned();
        }

        let (pfx_event_cnt, prefixes, victims, attackers)
            = process_pfx_events(value["pfx_events"].as_array().unwrap(), &event["event_type"].as_str().unwrap());

        event["pfx_events_cnt"] = json!(pfx_event_cnt);
        event["prefixes"] = json!(prefixes);
        event["victims"] = json!(victims);
        event["attackers"] = json!(attackers);
        res_vec.push(event);
    }
    res_vec
}

/// extract pfx events information:
/// - number of prefix events
/// - all prefixes
/// - victim ases
/// - attacker ases
fn process_pfx_events(pfx_events: &Vec<Value>, event_type: &str) -> (i32, Vec<String>, Vec<String>, Vec<String>){
    let mut pfx_event_cnt = 0;
    let mut prefixes:Vec<String> = vec!();
    let mut victims: Vec<String> = vec!();
    let mut attackers: Vec<String> = vec!();
    let mut checked = false;
    for pfx_event in pfx_events {
        match pfx_event["prefix"].as_str() {
            Some(p) => prefixes.push(p.to_owned()),
            _ => {}
        }
        match pfx_event["sub_pfx"].as_str() {
            Some(p) => prefixes.push(p.to_owned()),
            _ => {}
        }
        if !checked {
            checked = true;
            let (v, a) = extract_victims_attackers(pfx_event, event_type);
            victims.extend(v);
            attackers.extend(a);
        }
        pfx_event_cnt += 1;
    };

    (pfx_event_cnt, prefixes, victims, attackers)
}

/// extract victims and attackers from prefix event object
fn extract_victims_attackers(pfx_event: &Value, event_type: &str) -> (Vec<String>, Vec<String>) {
    let mut victims_set: HashSet<String> = HashSet::new();
    let mut attackers_set: HashSet<String> = HashSet::new();
    match event_type {
        "moas" => {
            attackers_set = pfx_event["newcomer_origins"].as_array().unwrap().iter()
                .map(|v| v.as_str().unwrap().to_owned()).collect();
            victims_set = pfx_event["origins"].as_array().unwrap().iter()
                .map(|v| v.as_str().unwrap().to_owned()).collect();
            victims_set.retain(|k| !attackers_set.contains(k));
        },
        "submoas" => {
            victims_set = pfx_event["sub_origins"].as_array().unwrap().iter()
                .map(|v| v.as_str().unwrap().to_owned()).collect();
            attackers_set = pfx_event["super_origins"].as_array().unwrap().iter()
                .map(|v| v.as_str().unwrap().to_owned()).collect();
        }
        "defcon" => {
            victims_set = pfx_event["origins"].as_array().unwrap().iter()
                .map(|v| v.as_str().unwrap().to_owned()).collect();
        }
        "edges" => {
            victims_set.insert(pfx_event["as1"].as_str().unwrap().to_owned());
            victims_set.insert(pfx_event["as2"].as_str().unwrap().to_owned());
        }
        _ => {}
    }

    let mut victims: Vec<String> = vec!();
    let mut attackers: Vec<String> = vec!();
    victims.extend(victims_set.into_iter());
    attackers.extend(attackers_set.into_iter());
    (victims, attackers)
}
