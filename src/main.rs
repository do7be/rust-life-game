extern crate rand;
use rand::Rng;
use std::{thread, time};

fn main() {
    let length = 10;
    let mut table: Vec<bool> = initilize_table(length);

    loop {
        clear();
        calculation_next_generation(&mut table, length);
        show_next_board(&table, length);
        sleep(2000);
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
    let mut rng = rand::thread_rng();
    for i in 0..(length * length) {
        table[i] = rng.gen();
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
