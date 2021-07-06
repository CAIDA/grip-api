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

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Header, Method};
use rocket::routes;
use rocket::serde::Deserialize;
use rocket::{Request, Response};

use grip_api::backend::api_external::*;
use grip_api::backend::api_json::*;
use grip_api::backend::api_stats::*;
use grip_api::backend::data::SharedData;

pub struct CORS();

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to requests",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON)
        {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, GET, OPTIONS",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }
        // if request.method() == Method::Options {
        //     response.set_header(ContentType::Plain);
        //     response.set_sized_body(Cursor::new(""));
        // }
    }
}

#[rocket::main]
#[allow(unused_must_use)]
async fn main() {
    let rocket = rocket::build();

    #[derive(Deserialize, Debug)]
    struct Config {
        address: String,
        port: u16,
        elastic_url: String,
    }

    let figment = rocket.figment();
    // let elastic_url =
    //     std::env::var("ELASTIC_URL").unwrap_or("http://clayface.caida.org:9200".to_string());
    let config: Config = figment
        .extract()
        .expect("failed to extract configuration parameters");

    dbg!(&config);

    rocket
        .mount(
            "/",
            routes![
                json_event_by_id,
                json_pfx_event_by_id,
                json_list_events,
                json_stats_today,
                json_stats_by_type,
                json_stats_total,
                json_get_tags,
                json_get_blacklist,
                json_get_blocklist,
                json_get_asndrop,
                json_get_hegemony,
                json_get_asrank,
            ],
        )
        .manage(SharedData {
            es_url: config.elastic_url,
        })
        .attach(CORS())
        .launch()
        .await;
}

#[cfg(test)]
mod test {
    use crate::get_rocket;
    use rocket::{http::Status, local::Client};

    #[test]
    fn test_basic_api() {
        let client = Client::new(get_rocket()).expect("valid rocket instance");
        let options = "/json/events?length=10&start=0&ts_start=2020-05-20T18%3A39&ts_end=2020-05-27T18%3A39&min_susp=80&max_susp=100&event_type=moas";
        let response = client.get(options).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
