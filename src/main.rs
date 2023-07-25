fn main() {
    let mut num = 0;

    loop {
        show_next_board(num);
        sleep(2000);
        num += 1;
    }
}

fn show_next_board(num: i32) {
    clear();

    println!("Hello, world! {}", num);
}

fn sleep(millisecond: u64) {
    use std::{thread, time};
    thread::sleep(time::Duration::from_millis(millisecond));
}

fn clear() {
    print!("{}[2J", 27 as char);
}
