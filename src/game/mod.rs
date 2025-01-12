#[derive(Debug, Clone)]
pub struct Tile {
    id: uuid::Uuid,
    y: i32,
    x: i32,
}

impl Tile {
    pub fn new(id: uuid::Uuid, y: i32, x: i32) -> Tile {
        Tile { id, y, x }
    }

    pub fn set_id(&mut self, id: uuid::Uuid) {
        self.id = id;
    }
    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }
    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn x(&self) -> i32 {
        self.x
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            id: uuid::Uuid::new_v4(),
            y: 0,
            x: 0,
        }
    }
}
