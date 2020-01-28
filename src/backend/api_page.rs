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
use serde_json::json;
use serde_json::Value;

use crate::backend::data::*;
use rocket::response::{NamedFile, Redirect};
use rocket_contrib::templates::Template;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

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
