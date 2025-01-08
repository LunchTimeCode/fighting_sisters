use maud::{html, Markup};
use rocket::{response::content, Route};

use crate::page;

#[get("/")]
pub fn body() -> content::RawHtml<String> {
    content::RawHtml(page::page(example()).into_string())
}

fn example() -> Markup {
    html! {

    body {

        div."navbar bg-base-100" {
            div."flex-1" {
                a."btn btn-ghost text-xl" {
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
                        select data-choose-theme {
                                option value="" {
                                    "Default"
                                }
                                option value="dark" {
                                    "Dark"
                                }
                                option value="light" {
                                    "Light"
                                }
                                option value="cupcake" {
                                    "Cupcake"
                                }
                            }
                    }
                }
            }
        }

        }

        img src="http://localhost:12500/_assets/png/isometric/barrel_E.png" alt="isometric" {}
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![body,])
}
