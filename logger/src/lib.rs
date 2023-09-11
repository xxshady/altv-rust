use std::cell::Cell;
use std::str::FromStr;

use log::{Level, LevelFilter, Log};

pub use log::debug;
pub use log::error;
pub use log::info;
pub use log::warn;

struct Logger {}

thread_local! {
    pub static LOG_IMPL: Cell<fn(&str)> = Cell::new(|_| {});
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
        let content = match record.level() {
            Level::Warn => format!("~yl~{content}"),
            Level::Error => format!("~rl~{content}"),
            _ => content,
        };

        LOG_IMPL.with(|v| {
            (v.get())(&format!("{module_path} {content}"));
        });
    }
    fn flush(&self) {}
}

pub fn init(log_impl: fn(&str)) -> Result<(), log::SetLoggerError> {
    let level = option_env!("LOG_LEVEL").unwrap_or("debug");

    LOG_IMPL.with(|v| {
        v.replace(log_impl);
    });

    log::set_max_level(LevelFilter::from_str(level).unwrap());
    log::set_logger(&Logger {})
}
