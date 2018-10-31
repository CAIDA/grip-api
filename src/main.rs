#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("gentelella/production/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    let path_str  = file.to_str().unwrap();

    // fix some paths for javascript objects
    let file_path = {
        if path_str.starts_with("vendors") {
            String::from(path_str).replace("vendors", "gentelella/vendors")
        } else if  path_str.starts_with("build"){
            String::from(path_str).replace("build", "gentelella/build")
        } else {
            let mut tmp = String::from("gentelella/production/");
            tmp.push_str(path_str);
            tmp
        }
    };

    NamedFile::open(Path::new(&file_path)).ok()
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, files])
}

fn main() {
    rocket().launch();
}

