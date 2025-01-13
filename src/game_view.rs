use grid::Grid;
use maud::{html, Markup};
use rocket::{response::content, Route};

use crate::{_State, events, game::Tile, htmx, tile_view};

#[get("/")]
async fn game(state: &_State) -> content::RawHtml<String> {
    let grid = state.get().await.grid();
    content::RawHtml(game_m(grid).into_string())
}

fn game_m(grid: Grid<Tile>) -> Markup {
    let grid_markup = grid_markup(grid);

    html! {
        div .container .mx-auto {
            (grid_markup)
        }

        div .container .mx-auto hx-get="/tile/debug" hx-trigger=(htmx::hx_event(events::ANY_TILE_SELECTED)) {
    
        }

    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/game", routes![game])
}

pub fn grid_markup(grid: grid::Grid<Tile>) -> Markup {
    let row_markup: Vec<Markup> = grid
        .iter_rows()
        .map(|row| row_markup(row.cloned().collect()))
        .collect();

    html! {
        div ."w-[1536]" ."h-[864]" {
            (column_markup(row_markup))
        }
    }
}

pub fn column_markup(rows: Vec<Markup>) -> Markup {
    html! {
        div .flex .flex-col {
            @for row in rows {
                (row)
            }
        }
    }
}

pub fn row_markup(row: Vec<Tile>) -> Markup {

    html! {
        div .flex .flex-row  {
            @for tile in row {
                (tile_view::tile_markup(tile, false))
            }
        }
    }
}
