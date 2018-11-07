#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate elastic_derive;
extern crate hijacks_dashboard;
extern crate rocket;
extern crate rocket_contrib;

use std::io;
use std::path::{Path, PathBuf};

use hijacks_dashboard::backend;
use hijacks_dashboard::backend::event_structs::Event;
use rocket::response::content;
use rocket::response::NamedFile;
use rocket_contrib::Json;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("app/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    let path_str = file.to_str().unwrap();
    let mut file_path = String::from("app/");
    file_path.push_str(path_str);

    NamedFile::open(Path::new(&file_path)).ok()
}

#[get("/example")]
fn example() -> Json<serde_json::Value> {
    // test ajax
    // content::Json(r#"{ "value": "ajax 10" }"#)
    let object = &backend::elastic::get_example_object().unwrap()[0];
    Json(object.to_owned())
}

#[derive(Serialize, Deserialize, Clone)]
struct Test {
    value: String,
}

#[get("/getten")]
fn getten() -> Json<Test> {
    // test ajax
    Json(Test{value: String::from("ajax 1000")})
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files, getten, example])
}

fn main() {
    rocket().launch();
}
