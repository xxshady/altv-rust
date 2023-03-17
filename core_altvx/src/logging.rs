use altv_sdk::ffi as sdk;

pub fn log(str: &str) {
    unsafe {
        sdk::ICore::LogColored(str, std::ptr::null_mut());
    }
}

pub fn log_error(str: &str) {
    unsafe {
        sdk::ICore::LogError(str, std::ptr::null_mut());
    }
}

pub fn log_warn(str: &str) {
    unsafe {
        sdk::ICore::LogWarning(str, std::ptr::null_mut());
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
