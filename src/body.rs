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
        }
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![body,])
}
