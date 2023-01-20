use once_cell::sync::OnceCell;
use std::sync::Mutex;

use crate::TestDataContainer;

pub type TimerId = u32;

struct Timer {
    id: TimerId,
    callback: Box<dyn FnMut() + 'static>,
    next_call_time: std::time::SystemTime,
    millis: u64,
    once: bool,
}

pub struct TimerManager {
    id: TimerId,
    timers: Vec<Timer>,
}

// TEST
unsafe impl Sync for TimerManager {}
unsafe impl Send for TimerManager {}

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

    pub fn create(
        &mut self,
        callback: Box<impl FnMut() + 'static>,
        millis: u64,
        once: bool,
    ) -> TimerId {
        let id = {
            self.id += 1;
            self.id
        };

        println!("create timer id: {:?}", id);

        let next_call_time =
            std::time::SystemTime::now() + std::time::Duration::from_millis(millis);

        self.timers.push(Timer {
            id,
            callback,
            next_call_time,
            millis,
            once,
        });

        dbg!(self.timers.len());

        id
    }

    pub fn process_timers(&mut self) {
        let now = std::time::SystemTime::now();
        let mut indexes_to_remove: Vec<usize> = vec![];

        for (idx, timer) in self.timers.iter_mut().enumerate() {
            if now >= timer.next_call_time {
                (timer.callback)();
                if timer.once {
                    println!("timer.once, millis: {}", timer.millis);
                    indexes_to_remove.push(idx);
                    continue;
                }
                timer.next_call_time =
                    std::time::SystemTime::now() + std::time::Duration::from_millis(timer.millis);
            }
        }

        for idx in indexes_to_remove {
            println!("deleting timer idx: {}", idx);
            self.timers.swap_remove(idx);
        }
    }
}
