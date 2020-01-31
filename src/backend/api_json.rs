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

#[get("/json/asndrop")]
pub fn json_get_asndrop() -> Json<Value> {
    let asndrop: Value = reqwest::get(format!("http://10.250.203.3:5000/asndrop").as_str())
        .unwrap()
        .json()
        .unwrap();
    Json(json!(asndrop))
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
