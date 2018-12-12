#![feature(proc_macro_hygiene)]

use rocket::fairing::AdHoc;
use rocket::routes;
use rocket_contrib::templates::Template;

use hijacks_dashboard::backend::api::*;

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                event_list,
                event_detail,
                traceroutes,
                files,
                json_event_by_id,
                json_pfx_event_by_id,
                json_list_events,
            ],
        )
        .attach(AdHoc::on_attach("get elastic search url", |rocket| {
            let base_url = rocket
                .config()
                .get_str("elastic_url")
                .unwrap_or("http://clayface.caida.org:9200")
                .to_string();
            Ok(rocket.manage(BaseUrl { url: base_url }))
        }))
        .attach(Template::fairing())
        .launch();
}
