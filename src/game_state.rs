use std::sync::Arc;

use grid::Grid;
use rocket::tokio::sync::Mutex;

use crate::game;

type LockedGameState = Arc<Mutex<GameState>>;

pub struct _GameState {
    state: LockedGameState,
}

impl Default for _GameState {
    fn default() -> Self {
        _GameState {
            state: Arc::new(Mutex::new(GameState::default())),
        }
    }
}

impl _GameState {
    pub async fn get(&self) -> GameState {
        self.state.lock().await.clone()
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    grid: grid::Grid<game::Tile>,
}

impl GameState {
    pub fn grid(&self) -> grid::Grid<game::Tile> {
        self.grid.clone()
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            grid: initail_grid(),
        }
    }
}

pub fn initail_grid() -> grid::Grid<game::Tile> {
    let mut grid: grid::Grid<game::Tile> = Grid::new(8, 16);
    grid.indexed_iter_mut().for_each(|((y, x), tile)| {
        tile.set_y(y as i32);
        tile.set_x(x as i32);
    });

    grid
}

pub fn initial_state() -> _GameState {
    _GameState::default()
}
