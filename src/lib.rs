mod life_game;
use life_game::LifeGame;
mod utils;
use wasm_bindgen::prelude::*;
extern crate rand;
extern crate web_sys;

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

impl Default for WasmLifeGame {
    fn default() -> Self {
        Self::new(10)
    }
}

#[wasm_bindgen]
pub struct WasmLifeGame {
    life_game: LifeGame,
}

#[wasm_bindgen]
impl WasmLifeGame {
    pub fn new(size: u32) -> Self {
        // panicをconsole.error()で書き出す処理。初期化時に1回だけ呼び出す。
        utils::set_panic_hook();

        Self {
            life_game: life_game::LifeGame::new(size),
        }
    }

    pub fn tick(&mut self) {
        self.life_game.next_generation();
        log!("tick");
    }

    pub fn render(&self) -> String {
        self.life_game.to_string()
    }

    pub fn size(&self) -> u32 {
        self.life_game.size
    }

    pub fn table(&self) -> js_sys::Array {
        self.life_game
            .table
            .borrow()
            .iter()
            .map(|&x| JsValue::from_bool(x))
            .collect()
    }
}
