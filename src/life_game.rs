extern crate rand;
use core::fmt;
use rand::Rng;
use std::cell::RefCell;

const MIN_SIZE: u32 = 10;
const MAX_SIZE: u32 = 100;

pub struct LifeGame {
    size: u32,
    table: RefCell<Vec<bool>>,
}

impl LifeGame {
    pub fn new(size: u32) -> Self {
        let size = std::cmp::min(std::cmp::max(size, MIN_SIZE), MAX_SIZE);
        let mut table: Vec<bool> = Vec::with_capacity(size as usize);
        let mut rng = rand::thread_rng();

        for _ in 0..(size * size) {
            table.push(rng.gen()); // 50%でtrue or false
        }

        Self {
            size,
            table: RefCell::new(table),
        }
    }

    pub fn next_generation(&self) {
        let current_table = self.table.borrow().clone(); // 変更前のテーブル
        let mut table = self.table.borrow_mut();

        for i in 0..(self.size * self.size) as usize {
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
        let size_i32 = self.size as i32;

        // 一次元で管理しているので左端、右端かどうか判定しておく必要がある
        let is_left_end = index_i32 % size_i32 == 0;
        let is_right_end = index_i32 % size_i32 == size_i32 - 1;

        self.count_alive_cell(table, index_i32 - (size_i32 + 1), is_left_end)
            // 上
            + self.count_alive_cell(table, index_i32 - size_i32, false)
            // 右上
            + self.count_alive_cell(table, index_i32 - (size_i32 - 1), is_right_end)
            // 左
            + self.count_alive_cell(table, index_i32 - 1, is_left_end)
            // 右
            + self.count_alive_cell(table, index_i32 + 1, is_right_end)
            // 左下
            + self.count_alive_cell(table, index_i32 + (size_i32 - 1), is_left_end)
            // 下
            + self.count_alive_cell(table, index_i32 + size_i32, false)
            // 右下
            + self.count_alive_cell(table, index_i32 + (size_i32 + 1), is_right_end)
    }

    fn count_alive_cell(
        &self,
        table: &[bool],
        index: i32,
        is_invalid_horizontal_edge: bool,
    ) -> i32 {
        if !(0..self.size * self.size).contains(&(index as u32)) || is_invalid_horizontal_edge {
            return 0;
        }

        if table[index as usize] {
            1
        } else {
            0
        }
    }

    #[allow(dead_code)]
    pub fn print_terminal(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for LifeGame {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.table
                .borrow()
                .iter()
                .enumerate()
                .fold(String::new(), |acc, (index, cell)| {
                    acc + if *cell { "■" } else { "□" }
                        + if (index + 1) % self.size as usize == 0 {
                            "\n"
                        } else {
                            " "
                        }
                })
        );
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let size = 11;
        let life_game = LifeGame::new(size);
        assert_eq!(life_game.size, size);
        assert_eq!(life_game.table.borrow().len(), (size * size) as usize);
    }

    #[test]
    fn test_new_smaller_min_size() {
        let size = 1;
        let life_game = LifeGame::new(size);
        assert_eq!(life_game.size, MIN_SIZE);
        assert_eq!(
            life_game.table.borrow().len(),
            (MIN_SIZE * MIN_SIZE) as usize
        );
    }

    #[test]
    fn test_new_larger_max_size() {
        let size = 101;
        let life_game = LifeGame::new(size);
        assert_eq!(life_game.size, MAX_SIZE);
        assert_eq!(
            life_game.table.borrow().len(),
            (MAX_SIZE * MAX_SIZE) as usize
        );
    }

    #[test]
    fn test_next_generation() {
        let life_game = LifeGame::new(10);
        let original = life_game.table.borrow().to_vec();
        life_game.next_generation();

        assert_ne!(original, life_game.table.borrow().to_vec());
    }
}
