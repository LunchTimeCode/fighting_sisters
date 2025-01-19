use maud::{html, Markup};
use rocket::{response::content, Route};

#[get("/")]
fn settings() -> content::RawHtml<String> {
    content::RawHtml(settings_m().into_string())
}

fn settings_m() -> Markup {
    html! {
        div .container .mx-auto {
                (settings_tabs(vec![
                    single_tab("General"),
                    single_tab("Sound")
                ]))
            }
    }
}

fn single_tab(tab_name: &str) -> Markup {
    let tab_name_name = tab_name.to_lowercase().to_string();
    html! {
            button .btn id=(tab_name_name) {
                (tab_name)
            }
    }
}

fn settings_tabs(tabs: Vec<Markup>) -> Markup {
    html! {
        div."container" {
            @for tab in &tabs {
                        (tab)
                }
        }
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/settings", routes![settings])
}
