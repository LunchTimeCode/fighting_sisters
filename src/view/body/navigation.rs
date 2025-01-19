use maud::{html, Markup};

use crate::view::components;

pub fn navigation() -> Markup {
    html! {
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
                        div."w-10 rounded-full"  {
                            (components::Cogs::render())
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

    }
}
