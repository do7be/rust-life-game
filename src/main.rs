mod life_game;
mod utils;
const SLEEP_MILLISECONDS: u64 = 500;

fn main() {
    let life_game = life_game::LifeGame::new(20);

    loop {
        utils::clear();
        life_game.next_generation();
        life_game.print();
        utils::sleep(SLEEP_MILLISECONDS);
    }
}
