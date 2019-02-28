use std::collections::HashMap;

use rocket::http::RawStr;
use rocket::State;
use rocket_contrib::templates::Template;
use serde_json::json;
use serde_json::Value;

use crate::backend::data::SharedData;

/*
LOAD HTML PAGES
*/

/// load events list page
#[get("/hi3/<_event_type>")]
pub fn hi3_page_event_list(_event_type: &RawStr, _data: State<SharedData>) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), json!({
        "onload_function":"load_events_table()" ,
    }));
    Template::render("hi3/event_list", context)
}

/// load event details page
#[get("/hi3/<_event_type>/<_id>")]
pub fn hi3_page_event_details(_event_type: &RawStr, _id: &RawStr, _data: State<SharedData>) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), json!({
        "onload_function":"load_event_details()",
    }));
    Template::render("hi3/event_detail", context)
}

/// load pfx_event details page
#[get("/hi3/<_event_type>/<_id>/<_pfx_finger_print>")]
pub fn hi3_page_traceroutes_page(_event_type: &RawStr, _id: &RawStr, _pfx_finger_print: &RawStr, _data: State<SharedData>) -> Template {
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), json!({
        "onload_function":"load_pfx_event()",
    }));
    Template::render("hi3/event_traceroutes", context)
}

