use maud::{html, Markup};
use rocket::{response::content::{self, RawHtml}, Route};

use crate::{_State, events::Coordinates, game::Tile, htmx::{self, Hxh}};

pub fn tile_markup(tile: Tile, selected: bool)-> Markup {
    let url = format!("/tile/select/{}/{}", tile.x(), tile.y());
    let selected_class = if selected { "border" } else { "" };
    html! {
    div
        .size-24 .flex-none .p-2

     .bg-no-repeat .bg-center .bg-local .bg-cover .bg-stone-tile .(selected_class)

        hx-post=(url) hx-swap="outerHTML" hx-trigger="click"
             {
                  (tile.id().to_string().split_at(3).0)

     p{
         "y" (tile.y())

     }
     p{
         "x" (tile.x())
     }
     }
    }.into()
}

#[post("/select/<x>/<y>")]
pub async fn select_tile(state: &_State, x: i32, y: i32) -> Hxh {
    let tile = state.get().await.select_tile(Coordinates::new(x, y));
    let tile = tile_markup(tile, true);
        
    Hxh::new("", Some(content::RawHtml(tile.into_string())))
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/tile", routes![select_tile])
}
