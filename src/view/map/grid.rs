use maud::{html, Markup};

use crate::{game::Tile, view::tile};

pub fn markup(grid: grid::Grid<Tile>) -> Markup {
    let row_markup: Vec<Markup> = grid
        .iter_rows()
        .map(|row| row_markup(row.cloned().collect()))
        .collect();

    html! {
        div ."w-[1336]" ."h-[764]" {
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
                (tile::markup(tile))
            }
        }
    }
}
