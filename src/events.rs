use rocket::Route;

#[derive(Debug)]
#[allow(unused)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
#[allow(unused)]
pub enum Event {
    Test(Coordinates),
}

#[get("/")]
pub async fn event() -> String {
    "Hello, world!".to_string()
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/next_event", routes![event])
}
