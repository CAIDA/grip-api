use rocket::http::RawStr;
use rocket::State;
use serde_json::json;
use serde_json::Value;

use crate::backend::data::*;
use rocket::response::{NamedFile, Redirect};
use std::path::{PathBuf, Path};
use std::collections::HashMap;
use rocket_contrib::templates::Template;

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
    Redirect::to("/events_suspicious/all")
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
            "onload_function":"load_events_table()",
            "debug": show_debug,
            "page_type": "event_list",
        }),
    );

    Template::render("event_list", context)
}

/// load events list page
#[get("/events_suspicious/<_event_type>?<debug>")]
pub fn page_suspicious_event_list(
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
            "onload_function":"load_events_table()",
            "debug": show_debug,
            "page_type": "event_list",
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
            "onload_function":"load_events_table()",
            "debug": show_debug,
            "page_type": "event_list",
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
            "onload_function":"load_events_table()",
            "debug": show_debug,
            "page_type": "event_list",
        }),
    );
    Template::render("event_list", context)
}

/// load events list page
#[get("/events_misconf/<_event_type>?<debug>")]
pub fn page_misconf_event_list(
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
            "onload_function":"load_events_table()",
            "debug": show_debug,
            "page_type": "event_list",
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
            "page_type": "event_details",
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
    let addrs: Vec<&str> = address_str.split('_').collect();
    let v: Vec<&str> = addrs[0].split("-").collect();
    let mut context = HashMap::<String, Value>::new();
    context.insert(
        "context".to_owned(),
        json!({
            "onload_function":"load_pfx_event()",
            "page_type": "pfx_event_details",
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
