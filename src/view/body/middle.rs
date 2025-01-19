use maud::{html, Markup};

use crate::events::{to_events, ADDED_CHARACTER, ANY_TILE_SELECTED};

pub fn middle() -> Markup {
    html! {
        div id="content"
        .container .mx-auto
        hx-get="/game"
        hx-trigger=(to_events(vec![
            ANY_TILE_SELECTED.to_owned()+ " from:body delay:200ms",
            "intersect once".into(),
            ADDED_CHARACTER.to_owned()+ " from:body delay:200ms"
        ]))
        {}
    }
}
