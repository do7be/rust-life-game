use std::{thread, time};
extern crate console_error_panic_hook;

pub fn sleep(millisecond: u64) {
    thread::sleep(time::Duration::from_millis(millisecond));
}

pub fn clear() {
    print!("{}[2J", 27 as char);
}

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}
