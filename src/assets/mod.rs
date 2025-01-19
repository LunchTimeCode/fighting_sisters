use rocket::{response::content, Route};

mod util;

#[get("/js/<name>")]
fn any_js(name: String) -> content::RawJavaScript<String> {
    content::RawJavaScript(
        util::read_any_file(name.as_str(), "js")
            .contents_utf8()
            .unwrap()
            .to_string(),
    )
}

#[get("/css/<name>")]
fn any_css(name: String) -> content::RawCss<String> {
    content::RawCss(
        util::read_any_file(name.as_str(), "css")
            .contents_utf8()
            .unwrap()
            .to_string(),
    )
}

#[derive(Responder)]
#[response(content_type = "image/png")]
struct ImageResponse(Vec<u8>);

#[get("/<path>/<name>")]
fn png(path: &str, name: &str) -> ImageResponse {
    let file = util::read_any_file(name, path);
    let bytes = file.contents();
    ImageResponse(bytes.to_vec())
}

#[derive(Responder)]
#[response(content_type = "image/x-icon")]
struct IconResponse(Vec<u8>);

#[get("/favicon/<name>")]
fn any_favicon(name: String) -> IconResponse {
    let file = util::read_any_file(name.as_str(), "favicon");
    let bytes = file.contents();
    IconResponse(bytes.to_vec())
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/_assets", routes![any_css, any_js, any_favicon, png])
}
