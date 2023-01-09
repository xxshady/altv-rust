use once_cell::sync::OnceCell;
use std::sync::Mutex;

use crate::TestDataContainer;

pub type TimerId = u32;

struct Timer {
    id: TimerId,
    callback: fn(),
    next_call_time: std::time::SystemTime,
    millis: u64,
    test_data: TestDataContainer,
}

pub struct TimerManager {
    id: TimerId,
    timers: Vec<Timer>,
}

// TEST
unsafe impl Sync for TimerManager {}

pub static TIMER_MANAGER: OnceCell<Mutex<TimerManager>> = OnceCell::new();

impl TimerManager {
    fn new() -> Self {
        dbg!("called new");

        Self {
            id: 0,
            timers: vec![],
        }
    }

    pub fn instance() -> &'static Mutex<TimerManager> {
        TIMER_MANAGER.get_or_init(|| Mutex::new(TimerManager::new()))
    }

    pub fn create(&mut self, callback: fn(), millis: u64, test_data: TestDataContainer) -> TimerId {
        let id = {
            self.id += 1;
            self.id
        };

        println!("create timer id: {:?}", id);

        println!("calling test_data callback...");
        (test_data.try_lock().unwrap().callback.try_lock().unwrap())("test");

        let next_call_time =
            std::time::SystemTime::now() + std::time::Duration::from_millis(millis);

        self.timers.push(Timer {
            id,
            callback,
            next_call_time,
            millis,
            test_data,
        });

        dbg!(self.timers.len());

        id
    }

    pub fn process_timers(&mut self) {
        let now = std::time::SystemTime::now();

        // dbg!(self.timers.len());

        for timer in &mut self.timers {
            // println!(
            //     "now: {:?} >= timer.next_call_time: {:?}",
            //     now, timer.next_call_time
            // );
            if now >= timer.next_call_time {
                println!("getting data");
                let data = timer.test_data.try_lock().unwrap();
                dbg!(&data.a);
                (data.callback.try_lock().unwrap())("test");
                println!("getted?");

                timer.next_call_time =
                    std::time::SystemTime::now() + std::time::Duration::from_millis(timer.millis);
            }
        }
    }
}
