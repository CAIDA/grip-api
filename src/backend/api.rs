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

use crate::backend::data::*;
use crate::backend::elastic::ElasticSearchBackend;
use crate::backend::utils::*;
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
#[get("/events/<_event_type>?<debug>")]
pub fn page_event_list(
    _event_type: &RawStr,
    debug: Option<bool>,
    _data: State<SharedData>,
) -> Template {
    let mut context = HashMap::<String, Value>::new();
    let show_debug = match debug {
        Some(b) => b,
        None => false,
    };
    context.insert(
        "context".to_owned(),
        json!({
            "debug": show_debug,
        }),
    );

    Template::render("event_list", context)
}

/// load events list page
#[get("/events_benign/<_event_type>?<debug>")]
pub fn page_benign_event_list(
    _event_type: &RawStr,
    debug: Option<bool>,
    _data: State<SharedData>,
) -> Template {
    let mut context = HashMap::<String, Value>::new();
    let show_debug = match debug {
        Some(b) => b,
        None => false,
    };
    context.insert(
        "context".to_owned(),
        json!({
            "debug": show_debug,
        }),
    );
    Template::render("event_list", context)
}

/// load events list page
#[get("/events_grey/<_event_type>?<debug>")]
pub fn page_grey_event_list(
    _event_type: &RawStr,
    debug: Option<bool>,
    _data: State<SharedData>,
) -> Template {
    let mut context = HashMap::<String, Value>::new();
    let show_debug = match debug {
        Some(b) => b,
        None => false,
    };
    context.insert(
        "context".to_owned(),
        json!({
            "debug": show_debug,
        }),
    );
    Template::render("event_list", context)
}

/// load event details page
#[get("/events/<_event_type>/<_id>?<debug>")]
pub fn page_event_details(
    _event_type: &RawStr,
    debug: Option<bool>,
    _id: &RawStr,
    _data: State<SharedData>,
) -> Template {
    let mut context = HashMap::<String, Value>::new();
    let show_debug = match debug {
        Some(b) => b,
        None => false,
    };
    context.insert(
        "context".to_owned(),
        json!({
            "onload_function":"load_event_details()",
            "debug": show_debug,
        }),
    );
    Template::render("event_detail", context)
}

/// load pfx_event details page
#[get("/events/<_event_type>/<_id>/<pfx_finger_print>")]
pub fn page_traceroutes_page(
    _event_type: &RawStr,
    _id: &RawStr,
    pfx_finger_print: &RawStr,
    _data: State<SharedData>,
) -> Template {
    dbg!(pfx_finger_print.to_string().replace("-", "\\/"));
    let address_str = pfx_finger_print.to_string();
    let v: Vec<&str> = address_str.split('-').collect();
    let mut context = HashMap::<String, Value>::new();
    context.insert(
        "context".to_owned(),
        json!({
            "onload_function":"load_pfx_event()",
            "address": v[0],
            "mask": v[1],
        }),
    );
    Template::render("event_traceroutes", context)
}

/// load events list page
#[get("/blacklist")]
pub fn page_blacklist(_data: State<SharedData>) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert(
        "context".to_owned(),
        json!({
            "onload_function":"load_blacklist()",
        }),
    );
    Template::render("blacklist", context)
}

/// load events list page
#[get("/tags")]
pub fn page_tags(_data: State<SharedData>) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert(
        "context".to_owned(),
        json!({
            "onload_function":"load_tags()",
        }),
    );
    Template::render("tags", context)
}

/*
JSON QUERY APIS
*/

#[get("/json/tags")]
pub fn json_get_tags() -> Json<Value> {
    let tags: Value = reqwest::get(format!("http://10.250.203.3:5000/tags/moas").as_str())
        .unwrap()
        .json()
        .unwrap();
    Json(json!(tags))
}

#[get("/json/blacklist")]
pub fn json_get_blacklist() -> Json<Value> {
    let blacklist: Value = reqwest::get(format!("http://10.250.203.3:5000/blacklist").as_str())
        .unwrap()
        .json()
        .unwrap();
    Json(json!(blacklist))
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
        Ok(event) => {
            let e = process_raw_event(&event.results[0], true);
            Json(json!(e))
        }
        Err(_e) => Json(json!({ "error": format!("Cannot find event {}", id) })),
    }
}

#[get("/json/pfx_event/id/<id>/<fingerprint>")]
pub fn json_pfx_event_by_id(
    id: &RawStr,
    fingerprint: &RawStr,
    base_url: State<SharedData>,
) -> Json<Value> {
    let backend_res = ElasticSearchBackend::new(&base_url.es_url);

    let backend = match backend_res {
        Ok(backend) => backend,
        Err(_e) => return Json(json!("Cannot connect to server")),
    };

    match backend.get_event_by_id(id) {
        Ok(event) => {
            match filter_pfx_events_by_fingerprint(fingerprint.as_str(), &event.results[0]) {
                Some(event) => Json(json!(event.to_owned())),
                None => Json(json!(
                    {
                        "error": "Cannot find prefix event"
                    }
                )),
            }
        }
        Err(_e) => Json(json!(
        {
            "error": "Cannot find event"
        }
        )),
    }
}

#[get("/json/events/<event_type>?<ts_start>&<ts_end>&<draw>&<start>&<length>&<asn>&<prefix>&<tags>&<min_susp>&<max_susp>")]
pub fn json_list_events(
    event_type: &RawStr,
    ts_start: Option<String>,
    ts_end: Option<String>,
    draw: Option<usize>,
    start: Option<usize>,
    length: Option<usize>,
    asn: Option<usize>,
    prefix: Option<String>,
    tags: Option<String>,
    min_susp: Option<usize>,
    max_susp: Option<usize>,
    base_url: State<SharedData>,
) -> Json<Value> {
    let backend = ElasticSearchBackend::new(&base_url.es_url).unwrap();
    let query_result = backend
        .list_events(
            event_type, &start, &length, &asn, &prefix, &ts_start, &ts_end,  &tags,
            &min_susp, &max_susp,
        )
        .unwrap();
    let res_data: Vec<Value> = query_result
        .results
        .iter()
        .map(|v| process_raw_event(v, false))
        .collect();
    let object = json!(
        {
            "data": res_data,
            // "data": query_result.results,
            "draw": draw,
            "recordsTotal": query_result.total,
            "recordsFiltered": query_result.total,
        }
    );
    // println!("{}", serde_json::to_string_pretty(&object).unwrap());
    Json(object.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_filtering() {
        let backend = ElasticSearchBackend::new(&"http://clayface.caida.org:9200").unwrap();
        let query_result = backend
            .list_events(
                &"moas",
                &Some(0),
                &Some(1),
                &None,
                &None,
                &None,
                &None,
                &None,
                &None,
                &None,
            )
            .unwrap();
        let res_data: Vec<Value> = query_result
            .results
            .iter()
            .map(|v| process_raw_event(v, false))
            .collect();
        println!("{}", serde_json::to_string_pretty(&res_data).unwrap());
        println!(
            "{}/{}",
            serde_json::to_string_pretty(&res_data).unwrap().len(),
            serde_json::to_string_pretty(&query_result.results)
                .unwrap()
                .len(),
        );
    }
}
