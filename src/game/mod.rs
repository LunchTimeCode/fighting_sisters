use character::Character;

pub mod character;
pub mod coordinates;
pub mod team;

#[derive(Debug, Clone, Default)]
pub struct Tile {
    y: i32,
    x: i32,
    selected: bool,
    character: Option<Character>,
}

impl Tile {
    pub fn new(y: i32, x: i32, selected: bool, character: Option<Character>) -> Tile {
        Tile {
            y,
            x,
            selected,
            character,
        }
    }

    pub fn set_character(&mut self, character: Character) {
        self.character = Some(character);
    }

    pub fn character(&self) -> Option<Character> {
        self.character.clone()
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
}
