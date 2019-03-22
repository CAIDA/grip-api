use serde_json::{json,Value};
use std::collections::HashSet;

/// shared state across rocket threads
pub struct SharedData {
    pub es_url: String,
    pub resource_dir: String,
}

/// process raw event from elasticsearch and convert the event into filtered data.
/// not all information is necessary for frontend processing
pub fn process_raw_event(value: &Value, include_tr: bool) -> Value {
    let mut event = json!({});
    // filter easy fields
    for field in vec!["event_type", "view_ts", "finished_ts", "duration", "external", "id", "tr_metrics"] {
        event[field] = value[field].to_owned();
    }

    let (pfx_events, prefixes, victims, attackers)
        = process_pfx_events(value["pfx_events"].as_array().unwrap(), &event["event_type"].as_str().unwrap(), include_tr);

    event["pfx_events"] = json!(pfx_events);
    event["prefixes"] = json!(prefixes);
    event["victims"] = json!(victims);
    event["attackers"] = json!(attackers);
    event["debug"] = extract_debug_info(value, &event);
    event
}

/// extract pfx events information:
/// - number of prefix events
/// - all prefixes
/// - victim ases
/// - attacker ases
fn process_pfx_events(value: &Vec<Value>, event_type: &str, include_tr: bool) -> (Vec<Value>, Vec<String>, Vec<String>, Vec<String>){
    let mut prefixes:Vec<String> = vec!();
    let mut victims: Vec<String> = vec!();
    let mut attackers: Vec<String> = vec!();
    let mut checked = false;
    let mut pfx_events: Vec<Value> = vec!();
    for raw_pfx_event in value {
        // extract attackers and victims
        if !checked {
            checked = true;
            let (v, a) = extract_victims_attackers(raw_pfx_event, event_type);
            victims.extend(v);
            attackers.extend(a);
        }

        let mut pfx_event = json!({});

        // build some basic fields
        for field in vec!["tags", "tr_worthy", "finished_ts"] {
            pfx_event[field] = raw_pfx_event[field].to_owned();
        }
        if include_tr {
            pfx_event["traceroutes"] = raw_pfx_event["traceroutes"].to_owned();
        }

        // set traceroute available
        if raw_pfx_event["traceroutes"].as_array().unwrap().len() > 0 {
            pfx_event["tr_available"] = json!(true);
        } else {
            pfx_event["tr_available"] = json!(false);
        }

        // set prefix and sub/super-prefix
        match raw_pfx_event["prefix"].as_str() {
            Some(p) => {
                prefixes.push(p.to_owned());
                pfx_event["prefix"] = raw_pfx_event["prefix"].to_owned();
            },
            _ => {}
        }
        match raw_pfx_event["sub_pfx"].as_str() {
            Some(p) => {
                prefixes.push(p.to_owned());
                pfx_event["sub_pfx"] = raw_pfx_event["sub_pfx"].to_owned();
                pfx_event["super_pfx"] = raw_pfx_event["super_pfx"].to_owned();
            },
            _ => {}
        }

        pfx_events.push(pfx_event);

        if pfx_events.len() > 20 {
            // do not display more than 20 pfx events
            break;
        }
    };

    (pfx_events, prefixes, victims, attackers)
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


pub fn extract_debug_info(raw_obj: &Value, processed_obj: &Value) -> Value {
    let raw_str = serde_json::to_string(raw_obj).unwrap();
    let processed_str = serde_json::to_string(processed_obj).unwrap();
    let mut debug = json!({});
    debug["raw_len"] = json!(raw_str.len());
    debug["processed_len"] = json!(processed_str.len());
    debug
}
