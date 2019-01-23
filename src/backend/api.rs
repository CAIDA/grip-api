use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use regex::Regex;
use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket::State;
use rocket::response::Redirect;
use rocket::request::Form;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde_json::json;
use serde_json::Value;

use crate::backend::elastic::ElasticSearchBackend;

pub struct BaseUrl {
    pub url: String,
}

#[get("/app/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    let path_str = file.to_str().unwrap();
    let mut file_path = String::from("app/");
    file_path.push_str(path_str);

    NamedFile::open(Path::new(&file_path)).ok()
}

/*
LOAD WEB PAGES
*/

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("/events/all")
}

#[get("/events/<event_type>")]
pub fn event_list(event_type: &RawStr) -> Template {
    let context_content = json!({"onload_function":format!("load_events_table('{}')",event_type) });
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), context_content);
    Template::render("event_list", context)
}

#[get("/event/<event_type>/<id>")]
pub fn event_detail(event_type: &RawStr, id: &RawStr, base_url: State<BaseUrl>) -> Template {
    let context_content =
        json!({ "onload_function": format!("{}()", "load_event_details") });
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), context_content);
    Template::render("event_detail", context)
}

#[get("/event/<event_type>/<id>/<pfx_finger_print>")]
pub fn traceroutes(event_type: &RawStr, id: &RawStr, pfx_finger_print: &RawStr, base_url: State<BaseUrl>) -> Template {
    let context_content =
        json!({ "onload_function": format!("{}()", "load_pfx_event") });

    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), context_content);
    Template::render(format!("{}", "event_traceroutes"), context)
}

/*
JSON QUERY APIS
*/

#[get("/json/event/id/<id>")]
pub fn json_event_by_id(id: &RawStr, base_url: State<BaseUrl>) -> Json<Value> {
    let backend_res = ElasticSearchBackend::new(&base_url.url);

    let backend = match backend_res {
        Ok(backend) => backend,
        Err(_e) => return Json(json!("Cannot connect to server")),
    };

    match backend.get_event_by_id(id) {
        Ok(event) => Json(json!({"data":event.results[0]["pfx_events"].to_owned()}).to_owned()),
        Err(_e) => Json(json!(
        {
            "error": format!("Cannot find event {}",id)
        }
        )),
    }
}

#[get("/json/pfx_event/id/<id>/<fingerprint>")]
pub fn json_pfx_event_by_id(id: &RawStr, fingerprint: &RawStr, base_url: State<BaseUrl>) -> Json<Value> {
    let backend_res = ElasticSearchBackend::new(&base_url.url);

    let backend = match backend_res {
        Ok(backend) => backend,
        Err(_e) => return Json(json!("Cannot connect to server")),
    };

    match backend.get_event_by_id(id) {
        Ok(event) => {
            match filter_pfx_events_by_fingerprint(fingerprint.as_str(), &event.results[0]) {
                Some(event) => {
                    Json(json!(event.to_owned()))
                }
                None => {
                    Json(json!(
                        {
                            "error": "Cannot find prefix event"
                        }
                    ))
                }
            }
        }
        Err(_e) => Json(json!(
        {
            "error": "Cannot find event"
        }
        )),
    }
}

#[get("/json/events/<event_type>?<ts_start>&<ts_end>&<draw>&<start>&<length>&<asn>&<prefix>")]
pub fn json_list_events(event_type: &RawStr, ts_start: Option<String>, ts_end: Option<String>,
                        draw: Option<usize>, start: Option<usize>, length: Option<usize>,
                        asn: Option<usize>, prefix: Option<String>,
                        base_url: State<BaseUrl>) -> Json<Value> {
    let backend = ElasticSearchBackend::new(&base_url.url).unwrap();
    let query_result = backend.list_events(event_type, &start, &length, &asn, &prefix, &ts_start, &ts_end).unwrap();
    let object = json!(
        {
            "data": query_result.results,
            "draw": draw,
            "recordsTotal": query_result.total,
            "recordsFiltered": query_result.total,
        }
    );
    Json(object.to_owned())
}

/*
    Utilities
*/

/// Find one specific prefix event from all prefix events in a event
fn filter_pfx_events_by_fingerprint<'a>(fingerprint: &str, event: &'a Value) -> Option<&'a Value> {
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

