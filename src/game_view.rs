use grid::Grid;
use maud::{html, Markup};
use rocket::{response::content, Route};

use crate::{_State, components, game::Tile, tile_view};

#[get("/")]
async fn game(state: &_State) -> content::RawHtml<String> {
    let state = state.get().await;
    let grid = state.grid();
    content::RawHtml(game_m(grid.clone()).into_string())
}

fn game_m(grid: Grid<Tile>) -> Markup {
    let grid_markup = grid_markup(grid);

    html! {
        div .flex .flex-row .mt-4 {

            div .container .mx-auto {
                (choosen_character_preview())
            }

            (components::Divider::horizontal(None).render())

            div .container .mx-auto {
                (grid_markup)
            }


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
                (tile_view::tile_markup(tile))
            }
        }
    }
}

fn choosen_character_preview() -> Markup {
    html! {
        div."card bg-base-100 w-96 shadow-xl" {
            div .bg-no-repeat .bg-center .bg-local .bg-cover .bg-ice-queen .size-56 {}

            div."card-body" {
                h2."card-title" {
                    "Cassandra!"
                }
                p {
                    (components::Progress::new_red(20,100).render())
                }
                p {
                    (components::Progress::new_yellow(20,100).render())
                }
                div."card-actions justify-end" {
                    button."btn btn-primary" {
                        "Fight"
                    }
                }
            }
        }
    }
}
