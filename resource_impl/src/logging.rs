use altv_sdk::ffi as sdk;
use cxx::let_cxx_string;

pub fn log(str: &str) {
    let_cxx_string!(cxx_str = str);
    unsafe {
        sdk::log_colored(&cxx_str);
    }
}

pub fn log_error(str: &str) {
    let_cxx_string!(cxx_str = str);
    unsafe {
        sdk::log_error(&cxx_str);
    }
}

pub fn log_warn(str: &str) {
    let_cxx_string!(cxx_str = str);
    unsafe {
        sdk::log_warn(&cxx_str);
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::logging::log(&format!($($arg)*))
    }}
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        $crate::logging::log_error(&format!($($arg)*))
    }}
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        $crate::logging::log_warn(&format!($($arg)*))
    }}
}
