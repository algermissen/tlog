extern crate log;
extern crate crossbeam;

use std::sync::mpsc;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;

use crossbeam::sync::MsQueue;


use log::{Log, LogLevel, LogLevelFilter, LogRecord, SetLoggerError, LogMetadata};

pub struct Logger {
    tx: Mutex<SyncSender<String>>
}

impl Log for Logger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let s = format!("{} - {}", record.level(), record.args());
            let mut data = self.tx.lock().unwrap();
            data.try_send(s).expect("Cannot log")
        }
    }
}

pub struct LockFreeLogger {
    tx: Arc<MsQueue<String>>
}
impl Log for LockFreeLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let s = format!("{} - {}", record.level(), record.args());
            self.tx.push(s);
        }
    }
}


pub fn init() -> Result<(), SetLoggerError> {
    let (tx, rx): (SyncSender<String>, Receiver<String>) = mpsc::sync_channel(100);

    //    thread::spawn(move || {
    //        let mut core = Core::new().unwrap();
    //        let f2 = rx.for_each(|res| {
    //            match res {
    //                s => {
    //                    //println!("{} - {}", record.level(), record.args());
    //                    println!("LOG {}", s);
    //                    Ok(())
    //                }
    //                s => {
    //                    println!("ERR");
    //                    Ok(())
    //                }
    //            }
    //        });
    //        core.run(f2).expect("Core failed to run");
    //    });

    thread::spawn(move || {
        loop {
            let s = rx.recv().unwrap();
            println!("V1 LOG: {}", s);
        }
    });

    log::set_logger(|max_log_level| {
        let logger = Logger {
            tx: Mutex::new(tx)
        };
        max_log_level.set(LogLevelFilter::Info);
        Box::new(logger)
    })
}
pub fn initLockFree() -> Result<(), SetLoggerError> {
    let q: Arc<MsQueue<String>> = Arc::new(MsQueue::new());

    let q2 = q.clone();

    thread::spawn(move || {
        loop {
            let s = q2.pop();
            println!("V1 LOG: {}", s);
        }
    });

    log::set_logger(|max_log_level| {
        let logger = LockFreeLogger {
            tx: q
        };
        max_log_level.set(LogLevelFilter::Info);
        Box::new(logger)
    })
}


//#[cfg(test)]
//mod tests {
//    #[test]
//    fn it_works() {
//        let logger = Logger {};
//        logger.log()
//    }
//}

