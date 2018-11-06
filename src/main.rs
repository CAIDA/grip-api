#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;
use rocket::response::content;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("app/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    let path_str  = file.to_str().unwrap();
    let mut file_path = String::from("app/");
    file_path.push_str(path_str);

    NamedFile::open(Path::new(&file_path)).ok()
}

#[get("/getten")]
fn getten() -> content::Json<&'static str>{
    // test ajax
    content::Json(r#"{ "value": "ajax 10" }"#)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files, getten])
}

fn main() {
    rocket().launch();
}

