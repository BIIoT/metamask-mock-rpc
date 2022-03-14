use chrono::NaiveDateTime;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

pub struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let now = chrono::Utc::now();
            // let str_time = now.format("%b %-d %-I:%M").to_string();
            let str_time = now.naive_utc().to_string();
            let mut s = str_time.split(".");
            println!("[{}/{}] {}", s.next().unwrap(), record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: Logger = Logger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}