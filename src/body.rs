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
            nav {
                ul {
                    li {
                        strong {
                            "Fighting Sisters"
                        }
                    }
                }
                ul {
                    li {
                        a."secondary" href="#" {
                            "Map"
                        }
                    }
                    li {
                        a."secondary" href="#" {
                            "Group"
                        }
                    }
                    li {
                        a."secondary" href="#" {
                            "Powers"
                        }
                    }
                    li {
                        details."dropdown" {
                            summary {
                                "Menu"
                            }
                            ul dir="rtl" {
                                li {
                                    a href="#" {
                                        "Profile"
                                    }
                                }
                                li {
                                    a href="#" {
                                        "Settings"
                                    }
                                }
                                li {
                                    a href="#" {
                                        "Save"
                                    }
                                }
                                li {
                                    a href="#" {
                                        "Info"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        img src="http://localhost:12500/_assets/png/barrel_E.png" alt="isometric" {}
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/", routes![body,])
}
