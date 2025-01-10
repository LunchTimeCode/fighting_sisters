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
                    single_tab(html!{
                        div {
                            h1 { "General" }
                            p { "This is the settings page" }
                        }
                    },"General", true),
                    single_tab(
                        html!{
                            div {
                                h1 { "Sound" }
                                p { "This is the settings page" }
                            }
                        },"Sound", false)
                ]))

            }

    }
}

fn single_tab(content: Markup, tab_name: &str, active: bool) -> Markup {
    let tab_name_name = tab_name.to_lowercase().to_string();
    html! {
            input type="radio" role="tab" ."tab" name=(tab_name_name) aria-label=(tab_name);
            div ."tab-content p-10" role="tabpanel" checked=(active) {
                (content)
            }
    }
}

fn settings_tabs(tabs: Vec<Markup>) -> Markup {
    html! {
        div."tabs tabs-bordered" role="tablist" {
            @for tab in &tabs {
                        (tab)
                }
        }
    }
}

pub fn api() -> (&'static str, Vec<Route>) {
    ("/settings", routes![settings])
}
