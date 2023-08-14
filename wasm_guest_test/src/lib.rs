mod resource;
mod timers;

use crate::{resource::Resource, timers::create_timer};

#[macro_export]
macro_rules! __log_error {
    ($($arg:tt)*) => {
        // $crate::api::log_error(&format!($($arg)*))
        todo!()
    };
}

pub use __log_error as log_error;

custom_print::define_macros!({ cprintln, cdbg }, concat, |output: String| guest::imports::log(
    output
));

wasm_bindgen::guest!("../wasm.interface");

impl guest::exports::Exports for guest::exports::ExportsImpl {
    fn main() {
        guest::imports::log_error("hello world".to_string());
        cprintln!("hello from cprintln");

        Resource::init();

        create_timer(
            Box::new(|| {
                println!("timer");
            }),
            1500,
            false,
        );
    }
}
