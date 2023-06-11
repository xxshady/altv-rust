use std::{cell::RefMut, fmt::Debug};

use crate::{resource::Resource, VoidResult};

pub type TimerId = u64;
pub type TimerCallback = dyn FnMut() -> VoidResult + 'static;

struct Timer {
    callback: Box<TimerCallback>,
    next_call_time: std::time::SystemTime,
    millis: u64,
    once: bool,
}

// derive(Debug) didn't work because of `callback: Box<TimerCallback>`
impl Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Timer{{}}")
    }
}

#[derive(Debug, Default)]
pub struct ScheduleState {
    id: TimerId,
    timers: Vec<Timer>,
}

impl ScheduleState {
    pub fn create(&mut self, callback: Box<TimerCallback>, millis: u64, once: bool) -> TimerId {
        let id = {
            self.id += 1;
            self.id
        };

        logger::debug!("creating timer with id: {id}");

        let next_call_time =
            std::time::SystemTime::now() + std::time::Duration::from_millis(millis);

        self.timers.push(Timer {
            callback,
            next_call_time,
            millis,
            once,
        });

        id
    }
}

#[derive(Debug, Default)]
pub struct TimerManager {
    timers: Vec<Timer>,
}

impl TimerManager {
    pub fn process_timers(&mut self, mut schedule: RefMut<ScheduleState>) {
        self.timers.append(&mut schedule.timers);
        drop(schedule); // unborrow ScheduleState

        let now = std::time::SystemTime::now();
        let mut indexes_to_remove: Vec<usize> = vec![];

        for (idx, timer) in self.timers.iter_mut().enumerate() {
            if now >= timer.next_call_time {
                if let Err(error) = (timer.callback)() {
                    logger::error!("timer callback failed with error: {error:?}");
                } else {
                    logger::debug!("timer callback called successfully");
                }
                if timer.once {
                    indexes_to_remove.push(idx);
                    continue;
                }
                timer.next_call_time =
                    std::time::SystemTime::now() + std::time::Duration::from_millis(timer.millis);
            }
        }

        for idx in indexes_to_remove {
            self.timers.swap_remove(idx);
        }
    }
}

pub fn create_timer(callback: Box<dyn FnMut() -> VoidResult + 'static>, millis: u64, once: bool) {
    Resource::with_timer_schedule_mut(|mut t, _| t.create(callback, millis, once));
}
