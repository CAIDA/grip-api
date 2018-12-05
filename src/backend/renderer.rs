use maud::Markup;
use maud::html;

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }

    pub fn render_test(&self) -> Markup {
        html! {
            p {"lala"}
        }
    }

    pub fn render_table(&self, entries: Vec<serde_json::Value>) -> Markup {
        html! {
            p {"lala"}
        }
    }
}