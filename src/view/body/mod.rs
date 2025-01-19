use maud::{html, Markup};
use rocket::{response::content, Route};

use crate::view::components;

mod left;
mod middle;
mod navigation;
mod page;

#[get("/")]
pub fn body() -> content::RawHtml<String> {
    content::RawHtml(page::page(body_m()).into_string())
}

fn body_m() -> Markup {
    html! {
    body {
    (navigation::navigation())
        div .container {
            div .flex .flex-row .mt-4 {

            (left::left())
            (components::Divider::horizontal(Some("h-[700px]")).render())
            (middle::middle())

            }
        }
        }
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![body,])
}
