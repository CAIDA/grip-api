use maud::html;
use maud::Markup;
use maud::PreEscaped;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }

    pub fn render_test(&self) -> Markup {
        html! {
        head{"test"}
        title {"BGPHijacks Dashboard"}

(PreEscaped("<script>alert(\"XSS\")</script>"))
            p {"lala"}
        }
    }

    pub fn render_table(&self, entries: Vec<serde_json::Value>) -> Markup {
        html! {
            p {"lala"}
        }
    }
}