use std::sync::Arc;

use grid::Grid;
use rocket::tokio::sync::Mutex;

use crate::events::Coordinates;
use crate::game::Tile;

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
    pub async fn get(&self) -> rocket::tokio::sync::MutexGuard<'_, GameState> {
        self.state.lock().await
    }
}

#[derive(Debug, Clone)]
pub struct GameState {
    grid: grid::Grid<Tile>,
}

impl GameState {
    pub fn grid(&self) -> &grid::Grid<Tile> {
        &self.grid
    }

    pub fn select_tile(&mut self, c: Coordinates) {
        self.grid.iter_mut().for_each(|tile| {
            tile.deselect();
        });
        let tile = self.grid.get_mut(c.y as usize, c.x as usize).unwrap();
        tile.select();
    }

    pub fn selected_tile(&self) -> Option<&Tile> {
        self.grid.iter().find(|x| x.selected())
    }

    pub fn get_tile(&self, c: Coordinates) -> Tile {
        self.grid.get(c.y as usize, c.x as usize).unwrap().clone()
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            grid: initial_grid(),
        }
    }
}

pub fn initial_grid() -> grid::Grid<Tile> {
    let mut grid: grid::Grid<Tile> = Grid::new(7, 14);
    grid.indexed_iter_mut().for_each(|((y, x), tile)| {
        tile.set_y(y as i32);
        tile.set_x(x as i32);
    });

    grid
}

pub fn initial_state() -> _GameState {
    _GameState::default()
}
