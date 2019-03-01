use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde_json::json;
use serde_json::Value;
use chrono::{Datelike, Timelike, Utc, Duration};

use crate::backend::elastic::ElasticSearchBackend;
use crate::backend::utils::*;
use crate::backend::data::SharedData;
use rocket::response::Redirect;


/// load static content
#[get("/app/<file..>")]
pub fn files(file: PathBuf, data: State<SharedData>) -> Option<NamedFile> {
    let path_str = file.to_str().unwrap();
    let mut file_path = data.resource_dir.clone();
    file_path.push_str("/app/");
    file_path.push_str(path_str);
    NamedFile::open(Path::new(&file_path)).ok()
}

/*
LOAD HTML PAGES
*/

/// load index page
#[get("/")]
pub fn page_index() -> Redirect {
    Redirect::to("/events/moas")
}

/// load events list page
#[get("/events/<_event_type>")]
pub fn page_event_list(_event_type: &RawStr, _data: State<SharedData>) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), json!({
        "onload_function":"load_events_table()" ,
    }));
    Template::render("event_list", context)
}

/// load event details page
#[get("/events/<_event_type>/<_id>")]
pub fn page_event_details(_event_type: &RawStr, _id: &RawStr, _data: State<SharedData>) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), json!({
        "onload_function":"load_event_details()",
    }));
    Template::render("event_detail", context)
}

/// load pfx_event details page
#[get("/events/<_event_type>/<_id>/<_pfx_finger_print>")]
pub fn page_traceroutes_page(_event_type: &RawStr, _id: &RawStr, _pfx_finger_print: &RawStr, _data: State<SharedData>) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), json!({
        "onload_function":"load_pfx_event()",
    }));
    Template::render("event_traceroutes", context)
}

/// load events list page
#[get("/blacklist")]
pub fn page_blacklist(_data: State<SharedData>) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), json!({
        "onload_function":"load_blacklist()",
    }));
    Template::render("blacklist", context)
}

/*
JSON QUERY APIS
*/

#[get("/json/tags")]
pub fn json_get_tags() -> Json<Value> {
    let tags: Value = reqwest::get(format!("http://10.250.203.3:5000/tags/moas").as_str()).unwrap().json().unwrap();
    Json(
        json!(tags)
    )
}

#[get("/json/blacklist")]
pub fn json_get_blacklist() -> Json<Value> {
    let blacklist: Value = reqwest::get(format!("http://10.250.203.3:5000/blacklist").as_str()).unwrap().json().unwrap();
    Json(
        json!(blacklist)
    )
}

#[get("/json/event/id/<id>")]
pub fn json_event_by_id(id: &RawStr, base_url: State<SharedData>) -> Json<Value> {
    let backend_res = ElasticSearchBackend::new(&base_url.es_url);

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
pub fn json_pfx_event_by_id(id: &RawStr, fingerprint: &RawStr, base_url: State<SharedData>) -> Json<Value> {
    let backend_res = ElasticSearchBackend::new(&base_url.es_url);

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

#[get("/json/asrank/<asn>")]
pub fn json_get_asrank(asn: usize) -> Json<Value> {
    Json(
        reqwest::get(format!("http://as-rank.caida.org/api/v1/asns/{}", asn).as_str())
            .unwrap().json().unwrap()
    )
}

#[get("/json/hegemony/<asn>")]
pub fn json_get_hegemony(asn: usize) -> Json<Value> {
    let now = Utc::now() - Duration::days(2);
    let time_str = format!("{}-{:02}-{:02}T{:02}:00", now.year(), now.month(), now.day(), now.hour());

    let url =
        format!(
            "https://ihr.iijlab.net/ihr/api/hegemony/?originasn=0&af=4&timebin={}&format=json&asn={}", time_str, asn
        );
    println!("{}", url);
    let hegemony: Value = reqwest::get(url.as_str()).unwrap().json().unwrap();
    Json(hegemony)
}



#[get("/json/events/<event_type>?<ts_start>&<ts_end>&<draw>&<start>&<length>&<asn>&<prefix>")]
pub fn json_list_events(event_type: &RawStr, ts_start: Option<String>, ts_end: Option<String>,
                        draw: Option<usize>, start: Option<usize>, length: Option<usize>,
                        asn: Option<usize>, prefix: Option<String>,
                        base_url: State<SharedData>) -> Json<Value> {
    let backend = ElasticSearchBackend::new(&base_url.es_url).unwrap();
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

