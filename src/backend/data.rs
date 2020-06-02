// This software is Copyright (c) 2018 The Regents of the University of
// California. All Rights Reserved. Permission to copy, modify, and distribute this
// software and its documentation for academic research and education purposes,
// without fee, and without a written agreement is hereby granted, provided that
// the above copyright notice, this paragraph and the following three paragraphs
// appear in all copies. Permission to make use of this software for other than
// academic research and education purposes may be obtained by contacting:
//
// Office of Innovation and Commercialization
// 9500 Gilman Drive, Mail Code 0910
// University of California
// La Jolla, CA 92093-0910
// (858) 534-5815
// invent@ucsd.edu
//
// This software program and documentation are copyrighted by The Regents of the
// University of California. The software program and documentation are supplied
// "as is", without any accompanying services from The Regents. The Regents does
// not warrant that the operation of the program will be uninterrupted or
// error-free. The end-user understands that the program was developed for research
// purposes and is advised not to rely exclusively on the program for any reason.
//
// IN NO EVENT SHALL THE UNIVERSITY OF CALIFORNIA BE LIABLE TO ANY PARTY FOR
// DIRECT, INDIRECT, SPECIAL, INCIDENTAL, OR CONSEQUENTIAL DAMAGES, INCLUDING LOST
// PROFITS, ARISING OUT OF THE USE OF THIS SOFTWARE AND ITS DOCUMENTATION, EVEN IF
// THE UNIVERSITY OF CALIFORNIA HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH
// DAMAGE. THE UNIVERSITY OF CALIFORNIA SPECIFICALLY DISCLAIMS ANY WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND
// FITNESS FOR A PARTICULAR PURPOSE. THE SOFTWARE PROVIDED HEREUNDER IS ON AN "AS
// IS" BASIS, AND THE UNIVERSITY OF CALIFORNIA HAS NO OBLIGATIONS TO PROVIDE
// MAINTENANCE, SUPPORT, UPDATES, ENHANCEMENTS, OR MODIFICATIONS.

use serde_json::{json, Map, Value};

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
        "id",
        "event_type",
        "view_ts",
        "finished_ts",
        "insert_ts",
        "last_modified_ts",
        "duration",
        "external",
        "tr_metrics",
        "event_metrics",
        "summary",
    ] {
        event[field] = value[field].to_owned();
    }

    let pfx_events = process_pfx_events(
        value["pfx_events"].as_array().unwrap(),
        include_tr,
        include_details,
    );

    event["pfx_events"] = json!(pfx_events);
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
        for field in vec!["tags", "finished_ts", "inferences"] {
            if let Some(value) = raw_pfx_event.get(field) {
                pfx_event[field] = value.to_owned();
            }
        }
        if include_tr {
            pfx_event["traceroutes"] = raw_pfx_event["traceroutes"]["msms"].to_owned();
        }
        if include_details {
            let mut details_map: Map<String, Value> =
                serde_json::from_value(raw_pfx_event["details"].to_owned()).unwrap();
            for field in vec!["sub_aspaths", "super_aspaths", "aspaths"] {
                details_map.remove(field);
            }
            pfx_event["details"] = Value::Object(details_map);
        }

        // set traceroute available
        pfx_event["tr_available"] = json!(raw_pfx_event["traceroutes"]["msms"]
            .as_array()
            .unwrap()
            .iter()
            .any(|msm| msm["results"].as_array().unwrap().len() > 0));
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

        // FIXME: the following check could explain why we're seeing less events in the ui.
        // if pfx_events.len() > 20 {
        //     // do not display more than 20 pfx events
        //     break;
        // }
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
