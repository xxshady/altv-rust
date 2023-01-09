pub mod timers;

use std::sync::{Arc, Mutex};

pub struct Managers {
    pub timer_manager: &'static Mutex<timers::TimerManager>,
}

pub struct ResourceApi {
    pub managers: Arc<Mutex<Managers>>,
}

pub type TestDataContainer = Arc<Mutex<TestData>>;
pub type TestDataCallbackContainer = Arc<Mutex<fn(&str)>>;

impl ResourceApi {
    pub fn create_timer(&self, callback: fn(), millis: u64, test_data: TestDataContainer) {
        self.managers
            .try_lock()
            .unwrap()
            .timer_manager
            .try_lock()
            .unwrap()
            .create(callback, millis, test_data);
    }
}

pub struct TestData {
    pub a: i32,
    pub callback: TestDataCallbackContainer,
}
