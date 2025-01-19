use maud::{html, Markup};
use rocket::{response::content, Route};

use crate::{page, view::components};

mod left;
mod middle;
mod navigation;

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
            (components::Divider::horizontal(None).render())
            (middle::middle())

            }
        }
        }
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![body,])
}
