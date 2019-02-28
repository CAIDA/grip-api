#![feature(proc_macro_hygiene)]

use rocket::fairing::AdHoc;
use rocket::routes;
use rocket_contrib::templates::Template;
use structopt::StructOpt;

use hijacks_dashboard::backend::api::*;
use hijacks_dashboard::backend::api_redirects::*;
use hijacks_dashboard::backend::data::SharedData;
use hijacks_dashboard::backend::data::get_tag_dict;
use std::collections::HashMap;
use rocket::Config;

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Opt {
    #[structopt(short = "d")]
    directory: Option<String>,
    #[structopt(short = "s")]
    simple: bool,
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
    extra_config.insert("simple_page".to_owned(), opts.simple.into());
    config.set_extras(extra_config);

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
                json_get_tags,
                json_get_blacklist,
                json_get_hegemony,
                json_get_asrank,
            ],
        )
        .attach(AdHoc::on_attach("get elastic search url", |rocket| {
            // set ElasticSearch URL
            let es_url = rocket.config().get_str("elastic_url")
                .unwrap_or("http://clayface.caida.org:9200") .to_string();
            let simple_page = rocket.config().get_extra("simple_page").unwrap().as_bool().unwrap_or(false);
            // pass in tags
            let tag_dict = get_tag_dict();
            Ok(rocket.manage(SharedData { es_url, tag_dict, resource_dir, simple_page}))
        }))
        .attach(Template::fairing())
        .launch();
}
