#[macro_use]
extern crate log;
extern crate tlog;

use std::time::Duration;
use std::thread;

fn main() {
    tlog::init_lock_free_logger().unwrap();
    info!("Test Message");
    thread::sleep(Duration::new(5, 0));
}

