use maud::{html, Markup};
use rocket::{response::content, Route};

use crate::{events::ANY_TILE_SELECTED, page};

#[get("/")]
pub fn body() -> content::RawHtml<String> {
    content::RawHtml(page::page(example()).into_string())
}

fn example() -> Markup {
    html! {

    body {
        div."navbar bg-base-100" {
            div."flex-1" {
                button ."btn btn-ghost text-xl" hx-get="/game" hx-target="#content" hx-trigger=(ANY_TILE_SELECTED.to_owned()+" from:body delay:200ms"+","+"click"+" ,"+"intersect once") {
                    "Fighting Sisters"
                }
            }
            div."flex-none" {

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

        div id="content" .container {

        }

        }


    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![body,])
}
