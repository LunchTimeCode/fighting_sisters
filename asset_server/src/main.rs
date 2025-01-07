use std::env;

use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

mod assets;

#[launch]
fn rocket() -> _ {
    env::set_var("ROCKET_port", "12501");
    env::set_var("ROCKET_address", "0.0.0.0");

    let rocket = rocket::build();
    mount(rocket)
}

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let (assets_path, asset_routes) = assets::api();
    rocket.mount(assets_path, asset_routes)
}
