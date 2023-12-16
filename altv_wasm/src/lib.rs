mod result;
pub use result::{IntoVoidResult, SomeResult, VoidResult};

mod logging;
pub use logging::{log, log_error, dbg};

mod timers;
pub use timers::{set_timeout, set_interval};

mod base_objects;
pub use base_objects::{
    objects::{vehicle::Vehicle, local_vehicle::LocalVehicle},
    remote::RemoteBaseObject,
    shared_vehicle::SharedVehicle,
    world_object::{WorldObject, ClientWorldObject},
    any_vehicle::AnyVehicle,
    local_vehicle::{LocalVehicleStatic, LocalVehicleStreamed},
};

mod hash;
pub use hash::{Hash, IntoHash, hash};

pub mod natives;

mod helpers;
mod state;
use crate::{
    state::State,
    base_objects::{objects::ObjectData, kind::BaseObjectKind},
};

mod memory_buffer;
pub use memory_buffer::{MemoryBuffer, MemoryBufferCreateError};

pub use shared::Vector3;
use altv_wasm_shared::{BaseObjectTypeRaw, BaseObjectType};

mod wasi_guest_gen;
#[doc(hidden)]
pub use wasi_guest_gen::{exports as __exports, imports as __imports};

mod asynch;
pub use asynch::{
    spawn as spawn_async,
    timer::{wait, wait_for},
};
pub use futures;

pub mod event;

#[no_mangle]
extern "C" fn __pre_main() {
    logger::init(|msg, level| {
        let msg = msg.to_string();

        use logger::Level as L;
        match level {
            L::Debug => __imports::log(&msg),
            L::Error => __imports::log_error(&msg),
            L::Warn => __imports::log_warn(&msg),
            L::Info => __imports::log(&msg),
            L::Trace => __imports::log(&msg),
        }
    })
    .unwrap();
    State::init();
}

impl __exports::Exports for __exports::ExportsImpl {
    fn on_tick() {
        State::with(|state| {
            // order is important, timers should be called before async executor
            state
                .timers
                .borrow_mut()
                .process_timers(state.timer_schedule.borrow_mut());
            state.async_executor.borrow_mut().run();
        });
    }

    fn on_base_object_create(ptr: altv_wasm_shared::BaseObjectPtr, ty: BaseObjectTypeRaw) {
        let ty = ty.try_into().unwrap();
        let kind = match ty {
            BaseObjectType::LocalVehicle => BaseObjectKind::LocalVehicleUnknown,
            _ => BaseObjectKind::Other,
        };

        State::with_base_objects_mut(|mut v, _| v.on_create(ptr, ty, kind));
    }

    fn on_base_object_destroy(ptr: altv_wasm_shared::BaseObjectPtr, ty: BaseObjectTypeRaw) {
        State::with_base_objects_mut(|mut v, _| v.on_destroy(ptr));
    }

    fn on_event(raw: altv_wasm_shared::RawEvent) {
        use altv_wasm_shared::RawEvent as RE;

        let context = match raw {
            RE::EnteredVehicle {
                vehicle: (ptr, ty),
                seat,
            } => event::contexts::EventContext::EnteredVehicle(event::contexts::EnteredVehicle {
                vehicle: match ty {
                    BaseObjectType::Vehicle => {
                        AnyVehicle::Vehicle(Vehicle::internal_new_borrowed(ptr))
                    }
                    BaseObjectType::LocalVehicle => {
                        State::with_base_objects_mut(|mut objects, _| match objects.all.get(&ptr) {
                            None => {
                                panic!("Unknown local vehicle ptr: {ptr}");
                            }
                            Some(ObjectData {
                                kind: BaseObjectKind::LocalVehicleUnknown,
                                ..
                            }) => {
                                AnyVehicle::LocalVehicle(LocalVehicle::internal_new_borrowed(ptr))
                            }
                            Some(_) => AnyVehicle::LocalVehicle(LocalVehicle::internal_new_owned(
                                ptr,
                                &mut objects.all,
                            )),
                        })
                    }
                    _ => panic!("unknown vehicle type: {ty:?}"),
                },
                seat,
            }),
        };

        State::with_events_mut(|mut events, _| {
            events.call_handlers(context);
        });
    }
}
