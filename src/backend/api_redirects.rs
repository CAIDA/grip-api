use rocket::response::Redirect;
use rocket::http::RawStr;

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to("/events/moas")
}

#[get("/event/<event_type>/<id>")]
pub fn event_details_old(event_type: &RawStr, id: &RawStr) -> Redirect {
    Redirect::permanent(format!("/events/{}/{}", event_type, id))
}

#[get("/event/<event_type>/<id>/<pfx_finger_print>")]
pub fn traceroutes_page_old(event_type: &RawStr, id: &RawStr, pfx_finger_print: &RawStr) -> Redirect {
    Redirect::to(format!("/events/{}/{}/{}", event_type, id, pfx_finger_print))
}
