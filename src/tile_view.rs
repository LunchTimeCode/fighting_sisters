use maud::{html, Markup};
use rocket::{response::content, Route};

use crate::{_State, events::Coordinates, game::Tile};

pub fn tile_markup(tile: Tile) -> Markup {
    let url = format!("/tile/select/{}/{}", tile.x(), tile.y());
    
    html! {
    div
        .size-24 .flex-none .p-2 .border

     .bg-no-repeat .bg-center .bg-local .bg-cover .bg-stone-tile

        hx-post=(url) hx-target="none"
             {
                  (tile.id().to_string().split_at(3).0)

     p{
         "y" (tile.y())

     }
     p{
         "x" (tile.x())
     }
     }
    }
}

#[post("/select/<x>/<y>")]
pub async fn select_tile(state: &_State, x: i32, y: i32) {
    state.get().await.select_tile(Coordinates::new(x, y));
    
}

pub fn tile_debug_m(tile: Tile) -> Markup {
    html! {
        div {
            p { "Tile ID: " (tile.id()) }
            p { "Tile y: " (tile.y()) }
            p { "Tile y: " (tile.x()) }
        }
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/tile", routes![select_tile])
}
