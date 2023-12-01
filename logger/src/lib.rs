use std::cell::Cell;
use std::str::FromStr;

use log::{LevelFilter, Log};

pub use log::{debug, error, info, warn, Level};

struct Logger {}

type OutputImpl = fn(&str, Level);

thread_local! {
    static LOG_IMPL: Cell<OutputImpl> = Cell::new(|_, _| {});
}

impl Log for Logger {
    fn enabled(&self, _: &log::Metadata) -> bool {
        true
    }
    fn log(&self, record: &log::Record) {
        let module_path = format!("[rust: {}]", record.module_path().unwrap());

        // skip logs from external crates
        if let Some(true) = record
            .file()
            .map(|v| v.starts_with('/') || v.starts_with("C:"))
        {
            return;
        }

        let content = record.args().to_string();

        LOG_IMPL.with(|v| {
            (v.get())(&format!("{module_path} {content}"), record.level());
        });
    }
    fn flush(&self) {}
}

pub fn init(log_impl: OutputImpl) -> Result<(), log::SetLoggerError> {
    let level = option_env!("LOG_LEVEL").unwrap_or("debug");

    LOG_IMPL.with(|v| {
        v.replace(log_impl);
    });

    log::set_max_level(LevelFilter::from_str(level).unwrap());
    log::set_logger(&Logger {})
}
