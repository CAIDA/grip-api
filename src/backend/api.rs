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
use crate::backend::utils::*;

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
LOAD HTML PAGES
*/


/// load events list page
#[get("/events/<event_type>")]
pub fn event_list(event_type: &RawStr) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), json!({"onload_function":"load_events_table()" }));
    Template::render("event_list", context)
}

/// load event details page
#[get("/events/<event_type>/<id>")]
pub fn event_details(event_type: &RawStr, id: &RawStr) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), json!({"onload_function":"load_event_details()"}));
    Template::render("event_detail", context)
}

/// load pfx_event details page
#[get("/events/<event_type>/<id>/<pfx_finger_print>")]
pub fn traceroutes_page(event_type: &RawStr, id: &RawStr, pfx_finger_print: &RawStr) -> Template {
    let mut context = HashMap::<String, Value>::new();
    Template::render("event_traceroutes", context)
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
        // Ok(event) => Json(json!({"data":event.results[0]["pfx_events"].to_owned()}).to_owned()),
        Ok(event) => Json(json!(event.results[0]).to_owned()),
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


