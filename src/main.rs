#![feature(proc_macro_hygiene)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate hijacks_dashboard;
extern crate rocket;
extern crate rocket_contrib;


use hijacks_dashboard::backend::api::*;
use rocket_contrib::Template;
use rocket::fairing::AdHoc;

fn main() {
    rocket::ignite()
        .mount("/", routes![index, files, json_event_by_id, json_all_events, ])
        .attach(AdHoc::on_attach(|rocket| {
            let base_url = rocket.config()
                .get_str("elastic_url")
                .unwrap_or("http://hammer.caida.org:9200")
                .to_string();
            Ok(rocket.manage(BaseUrl{url:base_url}))
        }))
        .attach( Template::fairing())
        .launch();
}
