pub const ANY_TILE_SELECTED: &str = "selected-tile";

#[derive(Debug, Clone, Default)]
#[allow(unused)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

impl Coordinates {
    pub fn new(x: i32, y: i32) -> Coordinates {
        Coordinates { x, y }
    }
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum EventType {
    SelectedTile(Coordinates),
}

impl EventType {
    pub fn hx_event(&self) -> String {
        match self {
            EventType::SelectedTile(c) => format!("selected-tile-{}-{}", c.x, c.y).clone(),
        }
    }
}

impl From<String> for EventType {
    fn from(value: String) -> Self {
        let mt = value.split("-").collect::<Vec<&str>>();

        match mt[0] {
            "selected" => match mt[1] {
                "tile" => {
                    let x = mt[2].parse::<i32>().unwrap();
                    let y = mt[3].parse::<i32>().unwrap();
                    EventType::SelectedTile(Coordinates::new(x, y))
                }
                _ => panic!("Unknown event type"),
            },
            _ => panic!("Unknown event type"),
        }
    }
}
