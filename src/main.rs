extern crate rand;
use rand::Rng;
mod utils;

const LENGTH: i32 = 20;
const SLEEP_MILLISECONDS: u64 = 500;

fn main() {
    let mut table: Vec<bool> = initilize_table();

    loop {
        utils::clear();
        calculate_next_generation(&mut table);
        show_next_board(&table);
        utils::sleep(SLEEP_MILLISECONDS);
    }
}

fn initilize_table() -> Vec<bool> {
    let mut table: Vec<bool> = Vec::with_capacity(LENGTH as usize);
    let mut rng = rand::thread_rng();

    for _ in 0..(LENGTH * LENGTH) {
        table.push(rng.gen()); // 50%でtrue or false
    }

    table
}

fn calculate_next_generation(table: &mut [bool]) {
    let current_table = table.to_vec(); // 変更前のテーブル

    for i in 0..(LENGTH * LENGTH) as usize {
        // 周囲に生きたセルがいくつあるか
        let count = count_around_cells(&current_table, i);
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

fn count_around_cells(table: &Vec<bool>, index: usize) -> i32 {
    // usizeの計算で0以下になる場合はpanicを起こすためキャストしている
    let index_i32 = index as i32;

    // 一次元で管理しているので左端、右端かどうか判定しておく必要がある
    let is_left_end = index_i32 % LENGTH == 0;
    let is_right_end = index_i32 % LENGTH == LENGTH - 1;

    let count =
        // 左上
        count_alive_cell(&table, index_i32 - (LENGTH + 1), is_left_end)
        // 上
        + count_alive_cell(&table, index_i32 - LENGTH, false)
        // 右上
        + count_alive_cell(&table, index_i32 - (LENGTH - 1), is_right_end)
        // 左
        + count_alive_cell(&table, index_i32 - 1, is_left_end)
        // 右
        + count_alive_cell(&table, index_i32 + 1, is_right_end)
        // 左下
        + count_alive_cell(&table, index_i32 + (LENGTH - 1), is_left_end)
        // 下
        + count_alive_cell(&table, index_i32 + LENGTH, false)
        // 右下
        + count_alive_cell(&table, index_i32 + (LENGTH + 1), is_right_end);

    count
}

fn count_alive_cell(table: &Vec<bool>, index: i32, is_invalid_horizontal_edge: bool) -> i32 {
    if index < 0 || index >= LENGTH * LENGTH || is_invalid_horizontal_edge {
        return 0;
    }

    if table[index as usize] {
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
                + if (index + 1) % LENGTH as usize == 0 {
                    "\n"
                } else {
                    " "
                })
    );
}
