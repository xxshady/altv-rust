#[macro_export]
macro_rules! __log {
  ($($arg:tt)*) => {
    $crate::wasm_imports::log_info(&format!($($arg)*))
  };
}
pub use __log as log_info;

#[macro_export]
macro_rules! __log_warn {
  ($($arg:tt)*) => {
    $crate::wasm_imports::log_warn(&format!($($arg)*))
  };
}
pub use __log_warn as log_warn;

#[macro_export]
macro_rules! __log_error {
  ($($arg:tt)*) => {
    $crate::wasm_imports::log_error(&format!($($arg)*))
  };
}
pub use __log_error as log_error;

custom_print::define_macro!(#[macro_export] cdbg, concat, $crate::wasm_imports::log_info);
pub use cdbg as dbg;
