extern crate rand;
use rand::Rng;
use std::{thread, time};

fn main() {
    let length = 20;
    let sleep_millisecond = 1000;

    let mut table: Vec<bool> = initilize_table(length);

    loop {
        clear();
        calculation_next_generation(&mut table, length);
        show_next_board(&table, length);
        sleep(sleep_millisecond);
    }
}

fn initilize_table(length: usize) -> Vec<bool> {
    let mut table: Vec<bool> = Vec::with_capacity(length);
    let mut rng = rand::thread_rng();

    for _ in 0..(length * length) {
        table.push(rng.gen()); // 50%でtrue or false
    }

    table
}

fn calculation_next_generation(table: &mut [bool], length: usize) {
    let current_table = table.to_vec(); // 変更前のテーブル

    for i in 0..(length * length) {
        // 周囲に生きたセルがいくつあるか
        let count = count_around_cells(current_table.to_vec(), i, length);
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

fn count_around_cells(table: Vec<bool>, index: usize, length: usize) -> i32 {
    let mut count: i32 = 0;

    // usizeの計算で0以下になる場合はpanicを起こすためキャストしている
    let index_i32 = index as i32;
    let length_i32 = length as i32;

    // 一次元で管理しているので左端、右端かどうか判定しておく必要がある
    let is_left_end = index_i32 % length_i32 == 0;
    let is_right_end = index_i32 % length_i32 == length_i32 - 1;

    // 左上
    if index_i32 - (length_i32 + 1) >= 0 && !is_left_end {
        count += count_alive_cell(table[index - (length + 1)]);
    }
    // 上
    if (index_i32 - length_i32) as i32 >= 0 {
        count += count_alive_cell(table[index - length]);
    }
    // 右上
    if (index_i32 - (length_i32 - 1)) as i32 >= 0 && !is_right_end {
        count += count_alive_cell(table[index - (length - 1)]);
    }
    // 左
    if (index_i32 - 1) as i32 >= 0 && !is_left_end {
        count += count_alive_cell(table[index - 1]);
    }
    // 右
    if index + 1 < length * length && !is_right_end {
        count += count_alive_cell(table[index + 1]);
    }
    // 左下
    if index + (length - 1) < length * length && !is_left_end {
        count += count_alive_cell(table[index + (length - 1)]);
    }
    // 下
    if index + length < length * length {
        count += count_alive_cell(table[index + length]);
    }
    // 右下
    if index + (length + 1) < length * length && !is_right_end {
        count += count_alive_cell(table[index + (length + 1)]);
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

fn show_next_board(table: &[bool], length: usize) {
    println!(
        "{}",
        table
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (index, cell)| acc
                + if *cell { "■" } else { "□" }
                + if (index + 1) % length == 0 { "\n" } else { " " })
    );
}

fn sleep(millisecond: u64) {
    thread::sleep(time::Duration::from_millis(millisecond));
}

fn clear() {
    print!("{}[2J", 27 as char);
}
