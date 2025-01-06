use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

mod assets;
mod body;
mod page;

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let rocket = rocket.configure(rocket::Config::figment().merge(("port", 12500)));
    let rocket = rocket.configure(rocket::Config::figment().merge(("address", "0.0.0.0")));

    mount(rocket)
}

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let (assets_path, asset_routes) = assets::api();
    let (body_path, body_routes) = body::api();
    rocket
        .mount(assets_path, asset_routes)
        .mount(body_path, body_routes)
}
