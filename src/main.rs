#![feature(proc_macro_hygiene)]

use rocket::fairing::AdHoc;
use rocket::routes;
use rocket::{Request, Response};
use rocket_contrib::templates::Template;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, ContentType, Method};
use std::io::Cursor;
use structopt::StructOpt;

use hijacks_dashboard::backend::api::*;
use hijacks_dashboard::backend::api_hi3::*;
use hijacks_dashboard::backend::api_stats::*;
use hijacks_dashboard::backend::data::SharedData;
use hijacks_dashboard::backend::data::get_tag_dict;
use std::collections::HashMap;
use rocket::Config;


pub struct CORS();

impl Fairing for CORS {

    fn info(&self) -> Info {
        Info { name: "Add CORS headers to requests", kind: Kind::Response }
    }

    fn on_response(&self, request: &Request, response: &mut Response) {
        if request.method() == Method::Options || response.content_type() == Some(ContentType::JSON) {
            response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
            response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, OPTIONS"));
            response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }
        if request.method() == Method::Options {
            response.set_header(ContentType::Plain);
            response.set_sized_body(Cursor::new(""));
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Opt {
    #[structopt(short = "d")]
    directory: Option<String>,
}

fn main() {
    let opts = Opt::from_args();
    let resource_dir: String;
    match opts.directory {
        Some(d) => resource_dir = d,
        None => resource_dir = "./".to_owned()
    };

    let mut config = Config::active().unwrap();
    let mut extra_config = HashMap::new();
    extra_config.insert("template_dir".to_owned(), format!("{}/templates",&resource_dir).into());
    config.set_extras(extra_config);
    config.set_address("0.0.0.0").unwrap();

    rocket::custom(config.clone())
        .mount(
            "/",
            routes![
                page_index,
                page_event_list,
                page_event_details,
                page_traceroutes_page,
                hi3_page_event_list,
                hi3_page_event_details,
                hi3_page_traceroutes_page,
                page_blacklist,
                files,
                json_event_by_id,
                json_pfx_event_by_id,
                json_list_events,
                json_stats_today,
                json_stats_total,
                json_get_tags,
                json_get_blacklist,
                json_get_hegemony,
                json_get_asrank,
            ],
        )
        .attach(CORS())
        .attach(AdHoc::on_attach("get elastic search url", |rocket| {
            // set ElasticSearch URL
            let es_url = rocket.config().get_str("elastic_url")
                .unwrap_or("http://clayface.caida.org:9200") .to_string();
            // pass in tags
            let tag_dict = get_tag_dict();
            Ok(rocket.manage(SharedData { es_url, tag_dict, resource_dir}))
        }))
        .attach(Template::fairing())
        .launch();
}
