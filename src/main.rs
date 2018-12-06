#![feature(proc_macro_hygiene)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

use rocket::fairing::AdHoc;
use rocket_contrib::Template;

use hijacks_dashboard::backend::api::*;

fn main() {
    rocket::ignite()
        .mount("/", routes![event_list, event_detail, files, json_event_by_id, json_all_events, ])
        .attach(AdHoc::on_attach(|rocket| {
            let base_url = rocket.config()
                .get_str("elastic_url")
                .unwrap_or("http://clayface.caida.org:9200")
                .to_string();
            Ok(rocket.manage(BaseUrl{url:base_url}))
        }))
        .attach( Template::fairing())
        .launch();
}
