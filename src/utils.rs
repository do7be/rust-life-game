use std::{thread, time};

pub fn sleep(millisecond: u64) {
    thread::sleep(time::Duration::from_millis(millisecond));
}

pub fn clear() {
    print!("{}[2J", 27 as char);
}
