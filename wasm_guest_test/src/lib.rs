mod resource;
mod timers;

use crate::{resource::Resource, timers::create_timer};

#[macro_export]
macro_rules! __log_error {
    ($($arg:tt)*) => {
        $crate::guest::imports::log_error(format!($($arg)*))
    };
}

pub use __log_error as log_error;

#[macro_export]
macro_rules! __log {
    ($($arg:tt)*) => {
        $crate::guest::imports::log(format!($($arg)*))
    };
}

pub use __log as log;

custom_print::define_macros!({ cprintln, cdbg }, concat, |output: String| guest::imports::log(
    output
));

wasm_codegen::guest!("../wasm.interface");

impl guest::exports::Exports for guest::exports::ExportsImpl {
    fn main() {
        guest::imports::log_error("hello world".to_string());
        cprintln!("hello from cprintln");

        Resource::init();

        create_timer(
            Box::new(|| {
                cprintln!("timer");
            }),
            1500,
            false,
        );
    }

    fn on_tick() {
        Resource::with_timers_mut(|mut timers, resource| {
            timers.process_timers(resource.timer_schedule.borrow_mut());
        });
    }
}
