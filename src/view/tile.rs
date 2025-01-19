use maud::{html, Markup};
use rocket::{
    response::content::{self},
    Route,
};

use crate::{
    _State,
    events::ANY_TILE_SELECTED,
    game::{coordinates::Coordinates, Tile},
    htmx::Hxh,
};

pub fn tile_markup(tile: Tile) -> Markup {
    let url = format!("/tile/select/{}/{}", tile.x(), tile.y());
    let selected_class = if tile.selected() {
        "bg-stone-tile-dark"
    } else {
        "bg-stone-tile"
    };

    html! {
            div .size-24 .flex-none .p-2 .bg-no-repeat .bg-center .bg-local .bg-cover .(selected_class)
            //---------htmx-----------
            hx-post=(url) hx-swap="outerHTML" hx-trigger="click" {
             (character_markup(tile))
             }
    }
}

pub fn character_markup(tile: Tile) -> Markup {
    if let Some(character) = tile.character() {
        html! {
                div .bg-no-repeat .bg-center .bg-local .bg-cover .(character.image()) .size-12 {}
        }
    } else {
        html! {}
    }
}

#[post("/select/<x>/<y>")]
pub async fn select_tile(state: &_State, x: i32, y: i32) -> Hxh {
    let mut state = state.get().await;
    state.select_tile(Coordinates::new(x, y));
    let tile = state.selected_tile().unwrap();

    let tile_view = tile_markup(tile.clone());

    Hxh::many(
        vec![ANY_TILE_SELECTED.to_owned()],
        Some(content::RawHtml(tile_view.into_string())),
    )
}

#[get("/get/<x>/<y>")]
async fn selected_tile(state: &_State, x: i32, y: i32) -> Hxh {
    let tile = state.get().await.get_tile(Coordinates::new(x, y));
    let tile_view = tile_markup(tile.clone());

    Hxh::new("", Some(content::RawHtml(tile_view.into_string())))
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/tile", routes![select_tile, selected_tile])
}
