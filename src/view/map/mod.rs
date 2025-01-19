use crate::{_State, game::Tile};
use ::grid::Grid;
use maud::{html, Markup};
use rocket::{response::content, Route};

mod grid;

fn markup(grid: Grid<Tile>) -> Markup {
    let grid_markup = grid::markup(grid);
    html! {
                (grid_markup)
    }
}

#[get("/")]
async fn map(state: &_State) -> content::RawHtml<String> {
    let state = state.get().await;
    let grid = state.grid();
    content::RawHtml(markup(grid.clone()).into_string())
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/game", routes![map])
}
