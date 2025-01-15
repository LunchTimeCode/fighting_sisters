use crate::events::{Coordinates, EventType};

#[derive(Debug, Clone, Default)]
pub struct Tile {
    y: i32,
    x: i32,
    selected: bool,
}

impl Tile {
    pub fn new(y: i32, x: i32, selected: bool) -> Tile {
        Tile { y, x, selected }
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn selected(&self) -> bool {
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self) {
        self.selected = false;
    }

    pub fn selected_event(&self) -> String {
        EventType::SelectedTile(Coordinates::new(self.x, self.y)).hx_event()
    }
}
