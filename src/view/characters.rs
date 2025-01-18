use maud::{html, Markup};
use rocket::{response::content, Route};

use crate::{
    _State, events::ADDED_CHARACTER, game::character::Character, htmx::Hxh, view::components,
};

fn choosen_character_preview(char: Character) -> Markup {
    html! {
        div."card bg-base-100 w-96 shadow-xl" {
            div .bg-no-repeat .bg-center .bg-local .bg-cover .(char.image()) .size-56 {}

            div."card-body" {
                h2."card-title" {
                   (char.name())
                }
                p {
                    (components::Progress::new_red(char.health(),100).render())
                }
                p {
                   ("Level: ")(char.level())
                }
                div."card-actions justify-end" {
                    button."btn btn-primary" {
                        "Fight"
                    }
                }
            }
        }
    }
}

fn no_character_selected() -> Markup {
    html! {
        div."card bg-base-100 w-96 shadow-xl" {
            div .bg-no-repeat .bg-center .bg-local .bg-cover .size-56 {}

            div."card-body" {
                h2."card-title" {
                   "No character selected"
                }
                p {
                    "Select a character to see more details"
                }
            }
        }
    }
}

#[get("/current")]
pub async fn current_char(state: &_State) -> content::RawHtml<String> {
    if let Some(char) = state.get().await.selected_tile().map(|c| c.character()) {
        if let Some(char) = char {
            return content::RawHtml(choosen_character_preview(char).into_string());
        }
        content::RawHtml(no_character_selected().into_string())
    } else {
        content::RawHtml(no_character_selected().into_string())
    }
}

#[post("/random")]
pub async fn add_random_char(state: &_State) -> Hxh {
    state.get().await.random_character();
    Hxh::only_header(ADDED_CHARACTER.to_owned())
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/chars", routes![current_char, add_random_char])
}
