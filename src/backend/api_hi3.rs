use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use chrono::{Datelike, Duration, Timelike, Utc};
use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::json::Json;
use rocket_contrib::templates::Template;
use serde_json::json;
use serde_json::Value;

use crate::backend::data::SharedData;
use crate::backend::elastic::ElasticSearchBackend;
use crate::backend::utils::*;

/*
LOAD HTML PAGES
*/

/// load events list page
#[get("/hi3/<_event_type>")]
pub fn hi3_page_event_list(_event_type: &RawStr, data: State<SharedData>) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), json!({
        "onload_function":"load_events_table()" ,
        "simple": data.simple_page
    }));
    Template::render("hi3/event_list", context)
}

/// load event details page
#[get("/hi3/<_event_type>/<_id>")]
pub fn hi3_page_event_details(_event_type: &RawStr, _id: &RawStr, data: State<SharedData>) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), json!({
        "onload_function":"load_event_details()",
        "simple": data.simple_page
    }));
    Template::render("hi3/event_detail", context)
}

/// load pfx_event details page
#[get("/hi3/<_event_type>/<_id>/<_pfx_finger_print>")]
pub fn hi3_page_traceroutes_page(_event_type: &RawStr, _id: &RawStr, _pfx_finger_print: &RawStr, data: State<SharedData>) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), json!({"onload_function":"load_pfx_event()",
        "simple": data.simple_page
    }));
    Template::render("hi3/event_traceroutes", context)
}

