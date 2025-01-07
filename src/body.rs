use maud::{html, Markup};
use rocket::{response::content, Route};

use crate::page;

#[get("/")]
pub fn body() -> content::RawHtml<String> {
    content::RawHtml(page::page(example()).into_string())
}

fn example() -> Markup {
    html! {

        body {
            main."container" {
                h1 {
                    "Hello world!"
                }
            }
            img src="http://localhost:12501/_assets/png/barrel_E" alt="isometric" {}
        }
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![body,])
}
