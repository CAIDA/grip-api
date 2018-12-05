#![plugin(rocket_codegen)]
#![feature(plugin)]
#![feature(proc_macro_hygiene)]

use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::path::PathBuf;

use maud::Markup;
use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket_contrib::Json;
use rocket_contrib::Template;

use backend::elastic::ElasticSearchBackend;
use backend::renderer::Renderer;

#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("app/index.html")
}

#[get("/app/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    let path_str = file.to_str().unwrap();
    let mut file_path = String::from("app/");
    file_path.push_str(path_str);

    NamedFile::open(Path::new(&file_path)).ok()
}

#[get("/json/<id>")]
pub fn json(id: &RawStr) -> Json<serde_json::Value> {
    let backend_res = ElasticSearchBackend::new("http://hammer.caida.org:9200");

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
pub fn example() -> Json<Vec<serde_json::Value>> {
    let backend = ElasticSearchBackend::new("http://hammer.caida.org:9200").unwrap();

    let object = backend.list_all_events().unwrap();
    Json(object.to_owned())
}

#[get("/maud")]
pub fn maud() -> Markup {
    let renderer = Renderer {};
    renderer.render_test()
}

#[get("/template")]
pub fn template() -> Template {
    let mut context = HashMap::<String, String>::new();
    context.insert("onload_function".to_owned(), "load_table()".to_owned());
    Template::render("index", context)
}
