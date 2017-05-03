extern crate log;
extern crate crossbeam;

use std::sync::mpsc;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::thread;
use std::sync::Arc;
use crossbeam::sync::MsQueue;
use log::{Log, LogLevel, LogLevelFilter, LogRecord, SetLoggerError, LogMetadata};

// Mutex based logger
pub struct MutexLogger {
    tx: Mutex<SyncSender<String>>
}

impl Log for MutexLogger {
    fn enabled(&self, metadata: &LogMetadata) -> bool {
        metadata.level() <= LogLevel::Info
    }

    fn log(&self, record: &LogRecord) {
        if self.enabled(record.metadata()) {
            let s = format!("{} - {}", record.level(), record.args());
            let data = self.tx.lock().unwrap();
            data.try_send(s).expect("Cannot log")
        }
    }
}

pub fn init_mutex_logger() -> Result<(), SetLoggerError> {
    let (tx, rx): (SyncSender<String>, Receiver<String>) = mpsc::sync_channel(100);
    thread::spawn(move || {
        loop {
            let s = rx.recv().unwrap();
            println!("Mutex Logger: {}", s);
        }
    });

    log::set_logger(|max_log_level| {
        let logger = MutexLogger {
            tx: Mutex::new(tx)
        };
        max_log_level.set(LogLevelFilter::Info);
        Box::new(logger)
    })
}

// Crossbeam MsQueue-based logger
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

pub fn init_lock_free_logger() -> Result<(), SetLoggerError> {
    let q: Arc<MsQueue<String>> = Arc::new(MsQueue::new());

    let q2 = q.clone();

    thread::spawn(move || {
        loop {
            let s = q2.pop();
            println!("Lock Free Logger: {}", s);
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






// Kept for further use (Tokio-based log processing)
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
