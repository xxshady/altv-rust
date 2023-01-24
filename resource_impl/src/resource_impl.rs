use crate::{
    events::{EventHandlersHashMap, SDKEvent, SDKEventManager},
    timers::{TimerCallback, TimerManager},
};
use once_cell::sync::OnceCell;
use std::sync::{Mutex, MutexGuard};

// should not be used in altv_module
static RESOURCE_IMPL_INSTANCE: OnceCell<Mutex<ResourceImpl>> = OnceCell::new();

#[derive(Debug)]
pub struct ResourceImpl {
    full_main_path: String,
    timers: TimerManager,
    sdk_events: SDKEventManager,
}

impl ResourceImpl {
    pub fn init(full_main_path: String) {
        let instance = ResourceImpl {
            full_main_path,
            timers: TimerManager::new(),
            sdk_events: SDKEventManager::new(),
        };

        RESOURCE_IMPL_INSTANCE
            .set(Mutex::new(instance))
            .expect("RESOURCE_IMPL_INSTANCE.set() failed");
    }

    pub fn get_mutex() -> &'static Mutex<Self> {
        RESOURCE_IMPL_INSTANCE.get().unwrap()
    }

    pub fn instance() -> MutexGuard<'static, Self> {
        RESOURCE_IMPL_INSTANCE
            .get()
            .expect("RESOURCE_IMPL_INSTANCE.get() failed")
            .try_lock()
            .expect("RESOURCE_IMPL_INSTANCE try_lock failed")
    }

    pub fn create_timer(&mut self, callback: Box<TimerCallback>, millis: u64, once: bool) {
        self.timers.create(callback, millis, once);
    }

    // intended for altv_module
    pub fn __on_tick(&mut self) {
        self.timers.__process_timers();
    }

    pub fn add_sdk_event_handler(&mut self, event: SDKEvent) {
        self.sdk_events.add_handler(event);
    }

    // intended for altv_module
    pub fn __get_sdk_event_handlers(&self) -> &EventHandlersHashMap {
        self.sdk_events.__get_handlers()
    }
}
