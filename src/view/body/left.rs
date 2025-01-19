use maud::{html, Markup};

use crate::events::{to_events, ANY_TILE_SELECTED};

pub fn left() -> Markup {
    html! {
        div id="left" .container .mx-auto
            hx-get="/chars/current"
        hx-trigger=(to_events(vec![
            ANY_TILE_SELECTED.to_owned() + " from:body delay:200ms",
            "intersect once".into()
        ]))
        {}
    }
}
