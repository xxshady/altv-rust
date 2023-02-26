use altv_sdk::ffi as sdk;

pub fn log(str: String) {
    unsafe {
        sdk::log_colored(str);
    }
}

pub fn log_error(str: String) {
    unsafe {
        sdk::log_error(str);
    }
}

pub fn log_warn(str: String) {
    unsafe {
        sdk::log_warn(str);
    }
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::logging::log(format!($($arg)*))
    }}
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {{
        $crate::logging::log_error(format!($($arg)*))
    }}
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {{
        $crate::logging::log_warn(format!($($arg)*))
    }}
}
