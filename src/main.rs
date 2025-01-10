use std::env;

use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

mod assets;
mod body;
mod game;
mod page;
mod settings;

#[launch]
fn rocket() -> _ {
    env::set_var("ROCKET_port", "12500");
    env::set_var("ROCKET_address", "0.0.0.0");

    let rocket = rocket::build();

    mount(rocket)
}

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let (assets_path, asset_routes) = assets::api();
    let (body_path, body_routes) = body::api();
    let (settings_path, settings_routes) = settings::api();
    let (game_path, game_routes) = game::api();
    rocket
        .mount(assets_path, asset_routes)
        .mount(body_path, body_routes)
        .mount(settings_path, settings_routes)
        .mount(game_path, game_routes)
}
