extern crate rand;
use rand::Rng;
use std::cell::RefCell;
mod utils;

const LENGTH: i32 = 20;
const SLEEP_MILLISECONDS: u64 = 500;

struct LifeGame {
    table: RefCell<Vec<bool>>,
}

impl LifeGame {
    pub fn new() -> Self {
        let mut table: Vec<bool> = Vec::with_capacity(LENGTH as usize);
        let mut rng = rand::thread_rng();

        for _ in 0..(LENGTH * LENGTH) {
            table.push(rng.gen()); // 50%でtrue or false
        }

        Self {
            table: RefCell::new(table),
        }
    }

    fn next_generation(&self) {
        let current_table = self.table.borrow().clone(); // 変更前のテーブル
        let mut table = self.table.borrow_mut();

        for i in 0..(LENGTH * LENGTH) as usize {
            // 周囲に生きたセルがいくつあるか
            let count = self.count_around_cells(&current_table, i);
            if table[i] {
                if count == 2 || count == 3 {
                    table[i] = true; // 生存
                } else if count <= 1 || count >= 4 {
                    table[i] = false; // 過疎 or // 過密
                }
            } else if count == 3 {
                table[i] = true; // 誕生
            }
        }
    }

    fn count_around_cells(&self, table: &[bool], index: usize) -> i32 {
        // usizeの計算で0以下になる場合はpanicを起こすためキャストしている
        let index_i32 = index as i32;

        // 一次元で管理しているので左端、右端かどうか判定しておく必要がある
        let is_left_end = index_i32 % LENGTH == 0;
        let is_right_end = index_i32 % LENGTH == LENGTH - 1;

        self.count_alive_cell(table, index_i32 - (LENGTH + 1), is_left_end)
            // 上
            + self.count_alive_cell(table, index_i32 - LENGTH, false)
            // 右上
            + self.count_alive_cell(table, index_i32 - (LENGTH - 1), is_right_end)
            // 左
            + self.count_alive_cell(table, index_i32 - 1, is_left_end)
            // 右
            + self.count_alive_cell(table, index_i32 + 1, is_right_end)
            // 左下
            + self.count_alive_cell(table, index_i32 + (LENGTH - 1), is_left_end)
            // 下
            + self.count_alive_cell(table, index_i32 + LENGTH, false)
            // 右下
            + self.count_alive_cell(table, index_i32 + (LENGTH + 1), is_right_end)
    }

    fn count_alive_cell(
        &self,
        table: &[bool],
        index: i32,
        is_invalid_horizontal_edge: bool,
    ) -> i32 {
        if !(0..LENGTH * LENGTH).contains(&index) || is_invalid_horizontal_edge {
            return 0;
        }

        if table[index as usize] {
            1
        } else {
            0
        }
    }

    fn print(&self) {
        println!(
            "{}",
            self.table
                .borrow()
                .iter()
                .enumerate()
                .fold(String::new(), |acc, (index, cell)| acc
                    + if *cell { "■" } else { "□" }
                    + if (index + 1) % LENGTH as usize == 0 {
                        "\n"
                    } else {
                        " "
                    })
        );
    }
}

fn main() {
    let life_game: LifeGame = LifeGame::new();

    loop {
        utils::clear();
        life_game.next_generation();
        life_game.print();
        utils::sleep(SLEEP_MILLISECONDS);
    }
}
