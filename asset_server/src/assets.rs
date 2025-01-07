use rocket::{fs::NamedFile, response::content, Route};

#[get("/pico.css")]
fn pico_css() -> content::RawCss<&'static str> {
    let app = include_str!("../assets/libs/pico.css");
    content::RawCss(app)
}

#[get("/js/htmx.min.js")]
fn htmx_code() -> content::RawJavaScript<&'static str> {
    let app = include_str!("../assets/libs/htmx.js");
    content::RawJavaScript(app)
}

#[get("/png/<name>")]
async fn png(name: String) -> NamedFile {
    let path = format!("assets/isometric/{}.png", name);
    NamedFile::open(path).await.unwrap()
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/_assets", routes![htmx_code, pico_css, png])
}
