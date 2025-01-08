use maud::{html, Markup};

use crate::assets;

pub fn page(markup: Markup) -> Markup {
    html! {
       html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="description" content="Fighting Sisters";
                ({assets::frontend::resources()})
                ({title("Fighting Sisters")})
            }

            body {
                (markup)
        }
       }
    }
}

fn title(title: impl Into<String>) -> Markup {
    html! {
    title { ({title.into()}) }
    }
}
