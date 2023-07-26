extern crate rand;
use rand::Rng;
mod utils;

const LENGTH: usize = 20;
const SLEEP_MILLISECONDS: u64 = 1000;

fn main() {
    let mut table: Vec<bool> = initilize_table();

    loop {
        utils::clear();
        calculation_next_generation(&mut table);
        show_next_board(&table);
        utils::sleep(SLEEP_MILLISECONDS);
    }
}

fn initilize_table() -> Vec<bool> {
    let mut table: Vec<bool> = Vec::with_capacity(LENGTH);
    let mut rng = rand::thread_rng();

    for _ in 0..(LENGTH * LENGTH) {
        table.push(rng.gen()); // 50%でtrue or false
    }

    table
}

fn calculation_next_generation(table: &mut [bool]) {
    let current_table = table.to_vec(); // 変更前のテーブル

    for i in 0..(LENGTH * LENGTH) {
        // 周囲に生きたセルがいくつあるか
        let count = count_around_cells(current_table.to_vec(), i);
        if table[i] {
            if count == 2 || count == 3 {
                table[i] = true; // 生存
            } else if count <= 1 {
                table[i] = false; // 過疎
            } else if count >= 4 {
                table[i] = false; // 過密
            }
        } else {
            if count == 3 {
                table[i] = true; // 誕生
            }
        }
    }
}

fn count_around_cells(table: Vec<bool>, index: usize) -> i32 {
    let mut count: i32 = 0;

    // usizeの計算で0以下になる場合はpanicを起こすためキャストしている
    let index_i32 = index as i32;
    let length_i32 = LENGTH as i32;

    // 一次元で管理しているので左端、右端かどうか判定しておく必要がある
    let is_left_end = index_i32 % length_i32 == 0;
    let is_right_end = index_i32 % length_i32 == length_i32 - 1;

    // 左上
    if index_i32 - (length_i32 + 1) >= 0 && !is_left_end {
        count += count_alive_cell(table[index - (LENGTH + 1)]);
    }
    // 上
    if (index_i32 - length_i32) as i32 >= 0 {
        count += count_alive_cell(table[index - LENGTH]);
    }
    // 右上
    if (index_i32 - (length_i32 - 1)) as i32 >= 0 && !is_right_end {
        count += count_alive_cell(table[index - (LENGTH - 1)]);
    }
    // 左
    if (index_i32 - 1) as i32 >= 0 && !is_left_end {
        count += count_alive_cell(table[index - 1]);
    }
    // 右
    if index + 1 < LENGTH * LENGTH && !is_right_end {
        count += count_alive_cell(table[index + 1]);
    }
    // 左下
    if index + (LENGTH - 1) < LENGTH * LENGTH && !is_left_end {
        count += count_alive_cell(table[index + (LENGTH - 1)]);
    }
    // 下
    if index + LENGTH < LENGTH * LENGTH {
        count += count_alive_cell(table[index + LENGTH]);
    }
    // 右下
    if index + (LENGTH + 1) < LENGTH * LENGTH && !is_right_end {
        count += count_alive_cell(table[index + (LENGTH + 1)]);
    }

    count
}

fn count_alive_cell(is_alive: bool) -> i32 {
    if is_alive {
        1
    } else {
        0
    }
}

fn show_next_board(table: &[bool]) {
    println!(
        "{}",
        table
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (index, cell)| acc
                + if *cell { "■" } else { "□" }
                + if (index + 1) % LENGTH == 0 { "\n" } else { " " })
    );
}
