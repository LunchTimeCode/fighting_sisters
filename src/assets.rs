use rocket::{response::content, Route};

#[get("/pico.css")]
fn pico_css() -> content::RawCss<&'static str> {
    let app = include_str!("../assets/libs/pico.css");
    content::RawCss(app)
}

#[get("/htmx.min.js")]
fn htmx_code() -> content::RawJavaScript<&'static str> {
    let app = include_str!("../assets/libs/htmx.js");
    content::RawJavaScript(app)
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/_assets", routes![htmx_code, pico_css,])
}

pub mod frontend {
    use maud::{html, Markup, PreEscaped};

    const PICO: &str = r#"<link rel="stylesheet" href="_assets/pico.css">"#;
    const HTMX: &str = r#"<link rel="stylesheet" href="_assets/htmx.js">"#;

    pub fn resources() -> Markup {
        html! {
        (PreEscaped(PICO))
        (PreEscaped(HTMX))
           }
    }
}
