mod resource;
mod result;
pub use result::{IntoVoidResult, SomeResult, VoidResult};

mod logging;
pub use logging::{log, log_error, dbg};

mod timers;
pub use timers::{set_timeout, set_interval};

use crate::resource::Resource;

wasm_codegen::guest!("../wasm.interface");

#[doc(hidden)]
pub use guest::exports as __exports;

#[doc(hidden)]
pub use guest::imports as __imports;

#[no_mangle]
extern "C" fn pre_main() {
    Resource::init();
}

impl __exports::Exports for __exports::ExportsImpl {
    fn on_tick() {
        Resource::with_timers_mut(|mut timers, resource| {
            timers.process_timers(resource.timer_schedule.borrow_mut());
        });
    }
}
