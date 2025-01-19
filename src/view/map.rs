use crate::{_State, game::Tile, view::tile};
use grid::Grid;
use maud::{html, Markup};
use rocket::{response::content, Route};

#[get("/")]
async fn game(state: &_State) -> content::RawHtml<String> {
    let state = state.get().await;
    let grid = state.grid();
    content::RawHtml(game_m(grid.clone()).into_string())
}

fn game_m(grid: Grid<Tile>) -> Markup {
    let grid_markup = grid_markup(grid);
    html! {
                (grid_markup)
    }
}

fn grid_markup(grid: grid::Grid<Tile>) -> Markup {
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

fn column_markup(rows: Vec<Markup>) -> Markup {
    html! {
        div .flex .flex-col {
            @for row in rows {
                (row)
            }
        }
    }
}

fn row_markup(row: Vec<Tile>) -> Markup {
    html! {
        div .flex .flex-row  {
            @for tile in row {
                (tile::tile_markup(tile))
            }
        }
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/game", routes![game])
}
