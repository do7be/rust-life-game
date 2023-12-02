mod life_game;
use life_game::LifeGame;
use wasm_bindgen::prelude::*;
extern crate rand;

impl Default for Universe {
    fn default() -> Self {
        Self::new(10)
    }
}

#[wasm_bindgen]
pub struct Universe {
    life_game: LifeGame,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(size: u32) -> Self {
        Self {
            life_game: life_game::LifeGame::new(size),
        }
    }

    pub fn tick(&mut self) {
        self.life_game.next_generation();
    }

    pub fn render(&self) -> String {
        self.life_game.to_string()
    }
}