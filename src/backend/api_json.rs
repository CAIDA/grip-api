use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::json;
use serde_json::Value;

use crate::backend::data::*;
use crate::backend::elastic::ElasticSearchBackend;
use crate::backend::utils::*;

/*
JSON QUERY APIS
*/

#[get("/json/tags")]
pub fn json_get_tags() -> Json<Value> {
    let tags: Value = reqwest::get(format!("http://10.250.203.3:5000/tags").as_str())
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
            let e = process_raw_event(&event.results[0], true, true);
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

#[get(
    "/json/events?\
     <event_type>&<ts_start>&<ts_end>&<draw>&<start>&<length>&<asns>&<pfxs>&\
     <tags>&<codes>&<min_susp>&<max_susp>&<misconf>&<misconf_type>&\
     <min_duration>&<max_duration>"
)]
pub fn json_list_events(
    event_type: Option<String>,
    ts_start: Option<String>,
    ts_end: Option<String>,
    draw: Option<usize>,
    start: Option<usize>,
    length: Option<usize>,
    asns: Option<String>,
    pfxs: Option<String>,
    tags: Option<String>,
    codes: Option<String>,
    min_susp: Option<usize>,
    max_susp: Option<usize>,
    misconf: Option<bool>,
    misconf_type: Option<String>,
    min_duration: Option<usize>,
    max_duration: Option<usize>,
    base_url: State<SharedData>,
) -> Json<Value> {
    let backend = ElasticSearchBackend::new(&base_url.es_url).unwrap();
    let query_result = backend
        .list_events(
            &event_type,
            &start,
            &length,
            &asns,
            &pfxs,
            &ts_start,
            &ts_end,
            &tags,
            &codes,
            &min_susp,
            &max_susp,
            &min_duration,
            &max_duration,
            &misconf,
            &misconf_type,
        )
        .unwrap();
    let res_data: Vec<Value> = query_result
        .results
        .iter()
        .map(|v| process_raw_event(v, false, false))
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
