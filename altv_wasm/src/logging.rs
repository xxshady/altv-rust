#[macro_export]
macro_rules! __log_error {
    ($($arg:tt)*) => {
        $crate::__imports::log_error(format!($($arg)*))
    };
}
pub use __log_error as log_error;

#[macro_export]
macro_rules! __log {
    ($($arg:tt)*) => {
        $crate::__imports::log(format!($($arg)*))
    };
}
pub use __log as log;

custom_print::define_macros!(#[macro_export] { cdbg }, concat, |output: String| $crate::__imports::log(
    output
));
pub use cdbg as dbg;
