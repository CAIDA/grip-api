use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use rocket::http::RawStr;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::Json;
use rocket_contrib::Template;

use backend::elastic::ElasticSearchBackend;
use serde_json::Value;

pub struct BaseUrl{
    pub url: String
}

#[get("/")]
pub fn index() -> Template {
    // NamedFile::open("app/index.html")
    let context_content = json!({"onload_function":"datatable_load()"});
    let mut context = HashMap::<String, Value>::new();
    context.insert("context".to_owned(), context_content);
    Template::render("index", context)
}

#[get("/app/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    let path_str = file.to_str().unwrap();
    let mut file_path = String::from("app/");
    file_path.push_str(path_str);

    NamedFile::open(Path::new(&file_path)).ok()
}

#[get("/event/<id>")]
pub fn event_query(id: &RawStr, base_url: State<BaseUrl>) -> Json<serde_json::Value> {
    // TODO: generate prefix events table
    let backend_res = ElasticSearchBackend::new(&base_url.url);

    let backend = match backend_res {
        Ok(backend) => backend,
        Err(_e) => return Json(json!("Cannot connect to server"))
    };

    match backend.get_event_by_id(id) {
        Ok(event) => Json(event.to_owned()),
        Err(_e) => Json(json!("Cannot find event"))
    }
}


/*
JSON QUERY APIS
*/

#[get("/json/event/id/<id>")]
pub fn json_event_by_id(id: &RawStr, base_url: State<BaseUrl>) -> Json<serde_json::Value> {
    let backend_res = ElasticSearchBackend::new(&base_url.url);

    let backend = match backend_res {
        Ok(backend) => backend,
        Err(_e) => return Json(json!("Cannot connect to server"))
    };

    match backend.get_event_by_id(id) {
        Ok(event) => Json(event.to_owned()),
        Err(_e) => Json(json!("Cannot find event"))
    }
}

#[get("/json/event/all/<max>")]
pub fn json_all_events(max: usize, base_url: State<BaseUrl>) -> Json<serde_json::Value> {
    let backend = ElasticSearchBackend::new(&base_url.url).unwrap();

    let object = json!({"data":backend.list_all_events(&max).unwrap()});
    Json(object.to_owned())
}
