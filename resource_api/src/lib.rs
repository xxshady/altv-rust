pub mod events;
pub mod timers;

use std::sync::{Arc, Mutex};

pub struct Managers {
    pub timer_manager: &'static Mutex<timers::TimerManager>,
    pub event_manager: &'static Mutex<events::SDKEventManager>,
}

pub struct ResourceApi {
    pub managers: Arc<Mutex<Managers>>,
}

pub type TestDataContainer = Arc<Mutex<TestData>>;
pub type TestDataCallbackContainer = Arc<Mutex<fn(&str)>>;

impl ResourceApi {
    pub fn create_timer(&self, callback: Box<impl FnMut() + 'static>, millis: u64, once: bool) {
        self.managers
            .try_lock()
            .unwrap()
            .timer_manager
            .try_lock()
            .unwrap()
            .create(callback, millis, once);
    }

    pub fn add_event_handler(&self, event: events::SDKEvent) {
        self.managers
            .try_lock()
            .unwrap()
            .event_manager
            .try_lock()
            .unwrap()
            .add_handler(event);
    }
}

pub struct TestData {
    pub a: i32,
    pub callback: TestDataCallbackContainer,
}
