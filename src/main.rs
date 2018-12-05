#![feature(proc_macro_hygiene)]
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate elastic_derive;
extern crate hijacks_dashboard;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate maud;

use std::io;
use std::path::{Path, PathBuf};

use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket_contrib::Json;

use hijacks_dashboard::backend;
use maud::Markup;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("app/index.html")
}

#[get("/app/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    let path_str = file.to_str().unwrap();
    let mut file_path = String::from("app/");
    file_path.push_str(path_str);

    NamedFile::open(Path::new(&file_path)).ok()
}

#[get("/json/<id>")]
fn json(id: &RawStr) -> Json<serde_json::Value> {
    let backend_res = backend::elastic::ElasticSearchBackend::new("http://hammer.caida.org:9200");

    let backend = match backend_res {
        Ok(backend) => backend,
        Err(_e) => return Json(json!("Cannot connect to server"))
    };

    match backend.get_event_by_id(id) {
        Ok(event) => Json(event.to_owned()),
        Err(_e) => Json(json!("Cannot find event"))
    }
}

#[get("/example")]
fn example() -> Json<Vec<serde_json::Value>> {
    let backend = backend::elastic::ElasticSearchBackend::new("http://hammer.caida.org:9200").unwrap();

    let object = backend.list_all_events().unwrap();
    Json(object.to_owned())
}

#[get("/maud")]
fn maud() -> Markup {
    let renderer = backend::renderer::Renderer{};
    renderer.render_test()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files, json, example, maud])
}

fn main() {
    rocket().launch();
}
