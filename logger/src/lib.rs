use std::str::FromStr;

use colored::*;
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
        let module_path = format!("[rust: {}]", record.module_path().unwrap()).bright_magenta();

        let content = record.args().to_string();
        let content = match record.level() {
            Level::Warn => content.bright_yellow(),
            Level::Error => content.bright_red(),
            _ => content.normal(),
        };

        println!("{module_path} {content}");
    }
    fn flush(&self) {}
}

pub fn init() -> Result<(), log::SetLoggerError> {
    let level = option_env!("LOG_LEVEL").unwrap_or("info");

    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();

    log::set_max_level(LevelFilter::from_str(level).unwrap());
    log::set_logger(&Logger {})
}
