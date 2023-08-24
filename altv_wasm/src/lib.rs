mod state;
mod result;
use altv_wasm_shared::BaseObjectType;
use altv_wasm_shared::BaseObjectTypeRaw;
pub use result::{IntoVoidResult, SomeResult, VoidResult};

mod logging;
pub use logging::{log, log_error, dbg};

mod timers;
pub use timers::{set_timeout, set_interval};

mod base_objects;
pub use base_objects::local_vehicle::{LocalVehicle, vehicles, VehicleManager};

mod hash;
pub use hash::{Hash, IntoHash, hash};

use crate::state::State;

wasm_codegen::guest!("../wasm.interface");

#[doc(hidden)]
pub use guest::exports as __exports;

#[doc(hidden)]
pub use guest::imports as __imports;

#[no_mangle]
extern "C" fn pre_main() {
    State::init();
}

impl __exports::Exports for __exports::ExportsImpl {
    fn on_tick() {
        State::with_timers_mut(|mut timers, state| {
            timers.process_timers(state.timer_schedule.borrow_mut());
        });
    }

    fn on_base_object_create(ptr: altv_wasm_shared::BaseObjectPtr, ty: BaseObjectTypeRaw) {
        State::with_base_objects_mut(|mut v, _| v.on_create(ptr, ty.try_into().unwrap()));
    }

    fn on_base_object_destroy(ptr: altv_wasm_shared::BaseObjectPtr, ty: BaseObjectTypeRaw) {
        State::with_base_objects_mut(|mut v, _| v.on_destroy(ptr, ty.try_into().unwrap()));
    }
}
