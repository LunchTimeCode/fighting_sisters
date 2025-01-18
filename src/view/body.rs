use maud::{html, Markup};
use rocket::{response::content, Route};

use crate::{
    events::{ADDED_CHARACTER, ANY_TILE_SELECTED},
    page,
    view::components,
};

#[get("/")]
pub fn body() -> content::RawHtml<String> {
    content::RawHtml(page::page(example()).into_string())
}

fn example() -> Markup {
    html! {

    body {
        div."navbar bg-base-100" {
            div."flex-1" {
                button ."btn btn-ghost text-xl"  {
                    "Fighting Sisters"
                }
            }
            div."flex-none" {
                button ."btn btn-ghost text-xl" hx-post="/chars/random" hx-trigger="click" hx-swap="none" {
                    "Add Random Char"
                }

                div."dropdown dropdown-end" {
                    div."btn btn-ghost btn-circle avatar" tabindex="0" role="button" {
                        div."w-10 rounded-full" {
                            img alt="Char" src="/_assets/png/portraits/silver_siren.png";
                        }
                    }
                    ul."menu menu-sm dropdown-content bg-base-100 rounded-box z-[1] mt-3 w-52 p-2 shadow" tabindex="0" {
                        li {
                                a."justify-between" {
                                    "Save"
                                }
                            }
                            li {
                                button
                                 hx-get="/settings"
                                 hx-target="#content"
                                {
                                    "Settings"
                                }
                            }
                            li {
                                a {
                                    "About"
                                }
                            }

                    }
                }
            }
        }

        div .container {
            div .flex .flex-row .mt-4 {

                div id="left" .container .mx-auto
                    hx-get="/chars/current"
                    hx-trigger=(
                        ANY_TILE_SELECTED.to_owned()+" from:body delay:200ms"+", "+"intersect once" ){} {}

                (components::Divider::horizontal(None).render())

                div
                id="content"
                .container .mx-auto
                hx-get="/game"
                hx-trigger=(ANY_TILE_SELECTED.to_owned()+" from:body delay:200ms"+", "+"intersect once"+ ", " +
                ADDED_CHARACTER + " from:body delay:200ms"){}

            }
        }

        }


    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![body,])
}
