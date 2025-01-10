use maud::{html, Markup};
use rocket::{response::content, Route};

#[get("/")]
fn game() -> content::RawHtml<String> {
    content::RawHtml(game_m().into_string())
}

fn game_m() -> Markup {
    html! {
              img src="/_assets/png/isometric/barrel_E.png" alt="isometric" {}
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/game", routes![game])
}
