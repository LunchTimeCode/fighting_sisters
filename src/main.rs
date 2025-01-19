use std::env;

use rocket::State;
use rocket::{Build, Rocket};

#[macro_use]
extern crate rocket;

mod assets;
mod events;
mod game;
mod game_state;
mod htmx;
mod view;

#[launch]
fn rocket() -> _ {
    env::set_var("ROCKET_port", "12500");
    env::set_var("ROCKET_address", "0.0.0.0");

    let rocket = rocket::build();

    mount(rocket)
}

pub type _State = State<game_state::_GameState>;

fn mount(rocket: Rocket<Build>) -> Rocket<Build> {
    let (assets_path, asset_routes) = assets::api();
    let (body_path, body_routes) = view::body::api();
    let (settings_path, settings_routes) = view::settings::api();
    let (game_path, game_routes) = view::map::api();
    let (tile_path, tile_routes) = view::tile::api();
    let (chars_path, chars_routes) = view::characters::api();
    let game_state = game_state::initial_state();
    rocket
        .mount(assets_path, asset_routes)
        .mount(body_path, body_routes)
        .mount(settings_path, settings_routes)
        .mount(game_path, game_routes)
        .mount(tile_path, tile_routes)
        .mount(chars_path, chars_routes)
        .manage(game_state)
}
