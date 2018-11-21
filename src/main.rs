#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate serde_derive;
extern crate elastic_derive;
extern crate hijacks_dashboard;
extern crate rocket;
extern crate rocket_contrib;

use std::io;
use std::path::{Path, PathBuf};

use hijacks_dashboard::backend;
use rocket::response::NamedFile;
use rocket::http::RawStr;
use rocket_contrib::Json;

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
fn json(id: &RawStr) -> Json<&str> {
    // let object = &backend::elastic::list_all_events().unwrap();
    // Json(object.to_owned())
    Json(id.as_str())
}

#[get("/example")]
fn example() -> Json<Vec<serde_json::Value>> {
    let object = &backend::elastic::list_all_events().unwrap();
    Json(object.to_owned())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files, json, example])
}

fn main() {
    rocket().launch();
}
