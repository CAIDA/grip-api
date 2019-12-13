use serde_json::{json, Value};

/// shared state across rocket threads
pub struct SharedData {
    pub es_url: String,
    pub resource_dir: String,
}

/// process raw event from elasticsearch and convert the event into filtered data.
/// not all information is necessary for frontend processing
pub fn process_raw_event(value: &Value, include_tr: bool, include_details: bool) -> Value {
    let mut event = json!({});
    // filter easy fields
    for field in vec![
        "event_type",
        "view_ts",
        "finished_ts",
        "duration",
        "external",
        "id",
        "tr_metrics",
        "tags",
        "inference",
        "summary",
    ] {
        event[field] = value[field].to_owned();
    }

    let pfx_events = process_pfx_events(
        value["pfx_events"].as_array().unwrap(),
        include_tr,
        include_details,
    );

    let victims = match event["inference"]["victims"].as_array() {
        Some(p) => p.to_owned(),
        _ => vec![],
    };
    let attackers = match event["inference"]["attackers"].as_array() {
        Some(p) => p.to_owned(),
        _ => vec![],
    };
    let prefixes = match event["summary"]["prefixes"].as_array() {
        Some(p) => p.to_owned(),
        _ => vec![],
    };

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
fn process_pfx_events(value: &Vec<Value>, include_tr: bool, include_details: bool) -> Vec<Value> {
    let mut prefixes: Vec<String> = vec![];
    let mut pfx_events: Vec<Value> = vec![];
    for raw_pfx_event in value {
        let mut pfx_event = json!({});

        // build some basic fields
        for field in vec!["tags", "finished_ts"] {
            pfx_event[field] = raw_pfx_event[field].to_owned();
        }
        if include_tr {
            pfx_event["traceroutes"] = raw_pfx_event["traceroutes"]["msms"].to_owned();
        }
        if include_details {
            pfx_event["details"] = raw_pfx_event["details"].to_owned();
        }

        // set traceroute available
        if raw_pfx_event["traceroutes"]["msms"]
            .as_array()
            .unwrap()
            .len()
            > 0
        {
            pfx_event["tr_available"] = json!(true);
        } else {
            pfx_event["tr_available"] = json!(false);
        }

        pfx_event["tr_worthy"] = raw_pfx_event["traceroutes"]["worthy"].to_owned();

        // set prefix and sub/super-prefix
        match raw_pfx_event["details"]["prefix"].as_str() {
            Some(p) => {
                prefixes.push(p.to_owned());
                pfx_event["prefix"] = raw_pfx_event["details"]["prefix"].to_owned();
            }
            _ => {}
        }
        match raw_pfx_event["details"]["sub_pfx"].as_str() {
            Some(p) => {
                prefixes.push(p.to_owned());
                pfx_event["sub_pfx"] = raw_pfx_event["details"]["sub_pfx"].to_owned();
                pfx_event["super_pfx"] = raw_pfx_event["details"]["super_pfx"].to_owned();
            }
            _ => {}
        }

        pfx_events.push(pfx_event);

        if pfx_events.len() > 20 {
            // do not display more than 20 pfx events
            break;
        }
    }

    pfx_events
}

pub fn extract_debug_info(raw_obj: &Value, processed_obj: &Value) -> Value {
    let raw_str = serde_json::to_string(raw_obj).unwrap();
    let processed_str = serde_json::to_string(processed_obj).unwrap();
    let mut debug = json!({});
    debug["raw_len"] = json!(raw_str.len());
    debug["processed_len"] = json!(processed_str.len());
    debug
}
