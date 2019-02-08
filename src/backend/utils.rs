/*
    Utilities
*/

use serde_json::Value;
use regex::Regex;

/// Find one specific prefix event from all prefix events in a event
pub fn filter_pfx_events_by_fingerprint<'a>(fingerprint: &str, event: &'a Value) -> Option<&'a Value> {
    let event_type = match event["event_type"].as_str() {
        Some(t) => t,
        None => return None
    };

    let re = Regex::new(r"-").unwrap();
    let result = re.replace_all(fingerprint, "/");

    let prefixes: Vec<&str> = result.split("_").collect();
    if prefixes.len() == 0 {
        return None;
    }

    let pfx_events: &Vec<Value> = match event["pfx_events"].as_array() {
        Some(events) => events,
        None => return None
    };

    match event_type {
        "moas" | "edges" => {
            if prefixes.len() != 1 {
                // must only have one prefix in the fingerprint for moas and edges cases
                return None;
            }

            for pfx_event in pfx_events {
                match pfx_event["prefix"].as_str() {
                    Some(pfx) => if pfx == prefixes[0] { return Some(&pfx_event); }
                    None => continue
                }
            }
            return None;
        }
        "submoas" | "defcon" => {
            if prefixes.len() != 2 {
                // must only have one prefix in the fingerprint for defcon and submoas cases
                return None;
            }

            for pfx_event in pfx_events {
                let sub_pfx = match pfx_event["sub_pfx"].as_str() {
                    Some(pfx) => pfx,
                    None => continue
                };
                let super_pfx = match pfx_event["super_pfx"].as_str() {
                    Some(pfx) => pfx,
                    None => continue
                };

                if sub_pfx == prefixes[0] && super_pfx == prefixes[1] {
                    // if we found the one
                    return Some(&pfx_event);
                }
            }
            return None;
        }
        _ => return None
    }
}
