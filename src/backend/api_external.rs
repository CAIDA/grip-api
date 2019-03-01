use rocket_contrib::json::Json;
use serde_json::Value;
use serde_json::json;
#[allow(unused_imports)] use chrono::{Datelike, Timelike, Utc, Duration};

fn request_for_json(url: &String) -> Value {
    match reqwest::get(url) {
        Ok(mut result) => match result.json() {
            Ok(j) => j,
            Err(e) => json!({"error": format!("{:?}",e)})
        },
        Err(e) => json!({"error": format!("{:?}",e)})
    }
}

#[get("/json/asrank/<asn>")]
pub fn json_get_asrank(asn: usize) -> Json<Value> {
    let request_url = format!("http://as-rank.caida.org/api/v1/asns/{}", asn);
    Json(
        request_for_json(&request_url)
    )
}

#[allow(dead_code)]
#[get("/json/hegemony/<asn>")]
pub fn json_get_hegemony(asn: usize) -> Json<Value> {
    let now = Utc::now() - Duration::days(2);
    let time_str = format!("{}-{:02}-{:02}T{:02}:00", now.year(), now.month(), now.day(), now.hour());

    let url =
        format!(
            "https://ihr.iijlab.net/ihr/api/hegemony/?originasn=0&af=4&timebin={}&format=json&asn={}", time_str, asn
        );
    println!("{}", url);
    Json(request_for_json(&url))
}



