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
}

fn main() {
    let opt = Opt::from_args();
    let resource_dir: String;
    match opt.directory {
        Some(d) => resource_dir = d,
        None => resource_dir = "./".to_owned()
    };

    let mut config = Config::active().unwrap();
    // set template directory
    let mut extra_config = HashMap::new();
    extra_config.insert("template_dir".to_owned(), format!("{}/templates",&resource_dir).into());
    config.set_extras(extra_config);

    rocket::custom(config.clone())
        .mount(
            "/",
            routes![
                index,
                event_list,
                event_details_old,      // backward compatible api route change
                traceroutes_page_old,   // backward compatible api route change
                event_details,
                traceroutes_page,
                files,
                json_event_by_id,
                json_pfx_event_by_id,
                json_list_events,
                json_get_tags,
                json_get_asrank,
            ],
        )
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
