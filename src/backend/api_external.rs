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

#[allow(unused_imports)]
use chrono::{Datelike, Duration, Timelike, Utc};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use serde_json::json;
use serde_json::Value;

fn request_for_json(url: &String) -> Value {
    match reqwest::get(url) {
        Ok(mut result) => match result.json() {
            Ok(j) => j,
            Err(e) => json!({ "error": format!("{:?}", e) }),
        },
        Err(e) => json!({ "error": format!("{:?}", e) }),
    }
}

#[get("/json/asrank/<asn>")]
pub fn json_get_asrank(asn: usize) -> Json<Value> {
    let request_url = format!("http://as-rank.caida.org/api/v1/asns/{}", asn);
    Json(request_for_json(&request_url))
}

#[allow(dead_code)]
#[get("/json/hegemony/<asn>")]
pub fn json_get_hegemony(asn: usize) -> Json<Value> {
    let now = Utc::now() - Duration::days(2);
    let time_str = format!(
        "{}-{:02}-{:02}T{:02}:00",
        now.year(),
        now.month(),
        now.day(),
        now.hour()
    );

    let url = format!(
        "https://ihr.iijlab.net/ihr/api/hegemony/?originasn=0&af=4&timebin={}&format=json&asn={}",
        time_str, asn
    );
    println!("{}", url);
    Json(request_for_json(&url))
}

/// load index page
#[get("/tags")]
pub fn page_tags_redirect() -> Redirect {
    Redirect::to("https://dev.hicube.caida.org/feeds/hijacks/tags")
}
