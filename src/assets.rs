use rocket::{response::content, Route};

#[get("/js/<path>")]
fn any_js(path: String) -> content::RawJavaScript<String> {
    let with_path = format!("http://localhost:12501/_assets/{}", path);
    let res = ureq::get(with_path.as_str()).call().unwrap();
    let body = res.into_string().unwrap();
    content::RawJavaScript(body)
}

#[get("/css/<path>")]
fn any_css(path: String) -> content::RawCss<String> {
    let with_path = format!("http://localhost:12501/_assets/{}", path);
    let res = ureq::get(with_path.as_str()).call().unwrap();
    let body = res.into_string().unwrap();
    content::RawCss(body)
}

#[derive(Responder)]
#[response(content_type = "image/png")]
struct ImageResponse(Vec<u8>);

#[get("/<path>")]
fn any_png(path: String) -> ImageResponse {
    let with_path = format!("http://localhost:12501/_assets/{}", path);
    let mut buf: Vec<u8> = Vec::new();
    ureq::get(with_path.as_str())
        .call()
        .unwrap()
        .into_reader()
        .read_to_end(&mut buf)
        .unwrap();
    ImageResponse(buf)
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/_assets", routes![any_css, any_js, any_png])
}

pub mod frontend {
    use maud::{html, Markup, PreEscaped};

    const PICO: &str = r#"<link rel="stylesheet" href="_assets/css/pico.css">"#;
    const HTMX: &str = r#"<link rel="stylesheet" href="_assets/js/htmx.js">"#;

    pub fn resources() -> Markup {
        html! {
        (PreEscaped(PICO))
        (PreEscaped(HTMX))
           }
    }
}
