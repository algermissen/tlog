#[macro_use]
extern crate log;
extern crate tlog;

//use std::process;
//use std::env;
//use std::str;
use std::time::Duration;
use std::thread;

fn main() {
    tlog::initLockFree().unwrap();
    info!("XYZ Message");
    thread::sleep(Duration::new(5, 0));
}

//fn run_child(rust_log: String) -> bool {
//    let exe = env::current_exe().unwrap();
//    let out = process::Command::new(exe)
//    .env("LOG_REGEXP_TEST", "1")
//    .env("RUST_LOG", rust_log)
//    .output()
//    .unwrap_or_else(|e| panic!("Unable to start child process: {}", e));
//    str::from_utf8(out.stderr.as_ref()).unwrap().contains("XYZ Message")
//}
//
//fn assert_message_printed(rust_log: &str) {
//    if !run_child(rust_log.to_string()) {
//        panic!("RUST_LOG={} should allow the test log message", rust_log)
//    }
//}
//
//fn assert_message_not_printed(rust_log: &str) {
//    if run_child(rust_log.to_string()) {
//        panic!("RUST_LOG={} should not allow the test log message", rust_log)
//    }
//}
//
//fn parent_main() {
//    // test normal log severity levels
//    assert_message_printed("info");
//    assert_message_not_printed("warn");
//
//    // test of regular expression filters
//    assert_message_printed("info/XYZ");
//    assert_message_not_printed("info/XXX");
//}
