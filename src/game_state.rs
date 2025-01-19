use std::sync::Arc;

use grid::Grid;
use rand::seq::SliceRandom;
use rocket::tokio::sync::Mutex;

use crate::game::character::Character;
use crate::game::coordinates::Coordinates;
use crate::game::{team, Tile};

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
    team: team::Team,
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

    pub fn random_character(&mut self) {
        self.team.add_random_char();
        let c = self.random_left_side_coordinates();
        let rand = self
            .team
            .characters()
            .choose(&mut rand::thread_rng())
            .unwrap();
        self.set_character(c, rand.clone());
    }

    pub fn set_character(&mut self, c: Coordinates, character: Character) {
        let tile = self.grid.get_mut(c.y as usize, c.x as usize).unwrap();
        tile.set_character(character);
    }

    pub fn random_left_side_coordinates(&self) -> Coordinates {
        let y = (0..7).collect::<Vec<i32>>()[rand::random::<usize>() % 7];
        let x = (0..14).collect::<Vec<i32>>()[rand::random::<usize>() % 7];
        Coordinates::new(y, x)
    }
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            grid: initial_grid(),
            team: team::Team::default(),
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
