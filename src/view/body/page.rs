use maud::{html, Markup};

pub fn page(markup: Markup) -> Markup {
    html! {
       html data-theme="dark" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
                meta name="description" content="Fighting Sisters";
                ({frontend::resources()})
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

pub mod frontend {
    use maud::{html, Markup, PreEscaped};

    const TAILWIND: &str = r#"<link href="./_assets/css/tw.css" rel="stylesheet">"#;
    const HTMX: &str = r#"<script src="/_assets/js/htmx.js"></script>"#;
    const FAVICON: &str =
        r#"<link rel="icon" type="image/x-icon" href="/_assets/favicon/favicon.ico">"#;
    const THEME_CHOOSER: &str = r#"<script src="/_assets/js/themechooser.js"></script>"#;

    pub fn resources() -> Markup {
        html! {
        (PreEscaped(HTMX))
        (PreEscaped(FAVICON))
        (PreEscaped(TAILWIND))
        (PreEscaped(THEME_CHOOSER))
           }
    }
}
