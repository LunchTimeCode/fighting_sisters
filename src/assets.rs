use include_directory::{include_directory, Dir, File};
use rocket::{response::content, Route};
use std::path::Path;

static PROJECT_DIR: Dir<'_> = include_directory!("assets");

fn read_any_file<'a>(name: &'a str, path: &'a str) -> File<'a> {
    let path = Path::new(path).join(name);
    let file = PROJECT_DIR.get_file(path.clone()).unwrap_or_else(|| {
        panic!(
            "could not find icon with this name: {}",
            path.to_str().unwrap()
        )
    });
    file.clone()
}

#[get("/js/<name>")]
fn any_js(name: String) -> content::RawJavaScript<String> {
    content::RawJavaScript(
        read_any_file(name.as_str(), "js")
            .contents_utf8()
            .unwrap()
            .to_string(),
    )
}

#[get("/css/<name>")]
fn any_css(name: String) -> content::RawCss<String> {
    content::RawCss(
        read_any_file(name.as_str(), "css")
            .contents_utf8()
            .unwrap()
            .to_string(),
    )
}

#[derive(Responder)]
#[response(content_type = "image/png")]
struct ImageResponse(Vec<u8>);

#[get("/png/<path>/<name>")]
fn any_png(path: String, name: String) -> ImageResponse {
    let file = read_any_file(name.as_str(), path.as_str());
    let bytes = file.contents();
    ImageResponse(bytes.to_vec())
}

#[get("/tiles/<name>")]
fn tiles(name: String) -> ImageResponse {
    let file = read_any_file(name.as_str(), "tiles");
    let bytes = file.contents();
    ImageResponse(bytes.to_vec())
}

#[derive(Responder)]
#[response(content_type = "image/x-icon")]
struct IconResponse(Vec<u8>);

#[get("/favicon/<name>")]
fn any_favicon(name: String) -> IconResponse {
    let file = read_any_file(name.as_str(), "favicon");
    let bytes = file.contents();
    IconResponse(bytes.to_vec())
}

pub fn api() -> (&'static str, Vec<Route>) {
    (
        "/_assets",
        routes![any_css, any_js, any_png, any_favicon, tiles],
    )
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
