use maud::{html, Markup};
use rocket::{response::content, Route};

use crate::{_State, game::Tile};

pub fn tile_markup(tile: Tile) -> Markup {
    html! {
    div
        .size-24 .flex-none .p-2 .border

     .bg-no-repeat .bg-center .bg-local .bg-cover .bg-stone-tile

        hx-post="/tile/debug" hx-trigger
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

#[post("/debug")]
pub async fn tile_debug(state: &_State) -> content::RawHtml<String> {
    let tile = state.get().await.grid().get(0, 0).unwrap().clone();
    content::RawHtml(tile_debug_m(tile).into_string())
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
    ("/tile", routes![tile_debug])
}
