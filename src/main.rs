extern crate rand;
use rand::Rng;
use std::{thread, time};

fn main() {
    let length = 3;
    let mut table: Vec<bool> = initilize_table(length);

    loop {
        clear();
        calculation_next_generation(&mut table, length);
        show_next_board(&table);
        sleep(2000);
    }
}

fn initilize_table(length: usize) -> Vec<bool> {
    let mut table: Vec<bool> = Vec::with_capacity(length);
    let mut rng = rand::thread_rng();

    for _ in 0..length {
        table.push(rng.gen()); // 50%でtrue or false
    }

    table
}

fn calculation_next_generation(table: &mut [bool], length: usize) {
    let mut rng = rand::thread_rng();
    for i in 0..length {
        table[i] = rng.gen();
    }
}

fn show_next_board(table: &[bool]) {
    println!(
        "{}",
        table.iter().fold(String::new(), |acc, cell| acc
            + if *cell { "■" } else { "□" }
            + " ")
    );
}

fn sleep(millisecond: u64) {
    thread::sleep(time::Duration::from_millis(millisecond));
}

fn clear() {
    print!("{}[2J", 27 as char);
}
