use maud::{html, Markup};

#[derive(Debug, Clone, Default)]
pub struct Progress {
    classes: String,
    value: i32,
    max: i32,
}

#[allow(unused)]
impl Progress {
    pub fn new(classes: &str, value: i32, max: i32) -> Progress {
        Progress {
            classes: classes.to_string(),
            value,
            max,
        }
    }

    pub fn new_red(value: i32, max: i32) -> Progress {
        Progress {
            classes: "progress-error".to_string(),
            value,
            max,
        }
    }

    pub fn new_yellow(value: i32, max: i32) -> Progress {
        Progress {
            classes: "progress-warning".to_string(),
            value,
            max,
        }
    }

    pub fn render(&self) -> Markup {
        let Progress {
            classes,
            value,
            max,
        } = self;
        html! {
            progress .progress .(classes) value=(value) max=(max) {}
        }
    }
}

pub struct Divider {
    classes: String,
    orientation: String,
}

#[allow(unused)]
impl Divider {
    pub fn new(classes: &str, orientation: &str) -> Divider {
        Divider {
            classes: classes.to_string(),
            orientation: orientation.to_string(),
        }
    }

    pub fn horizontal(classes: Option<&str>) -> Divider {
        let classes = classes.unwrap_or("");
        Divider {
            classes: classes.to_string(),
            orientation: "divider-horizontal".to_string(),
        }
    }

    pub fn render(&self) -> Markup {
        let Divider {
            classes,
            orientation,
        } = self;
        html! {
            div ."divider" .(orientation) .(classes) {}
        }
    }
}
