use std::str::FromStr;

use log::{Level, LevelFilter, Log};

pub use log::debug;
pub use log::error;
pub use log::info;
pub use log::warn;

struct Logger {}

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

        altv_sdk::helpers::log(&format!("{module_path} {content}"));
    }
    fn flush(&self) {}
}

pub fn init() -> Result<(), log::SetLoggerError> {
    let level = option_env!("LOG_LEVEL").unwrap_or("info");

    log::set_max_level(LevelFilter::from_str(level).unwrap());
    log::set_logger(&Logger {})
}
