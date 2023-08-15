use std::{fmt::Debug, time, cell::RefMut};

use crate::{log, log_error, resource::Resource};

pub type TimerId = u64;
pub type TimerCallback = dyn FnMut() + 'static;

struct TimerData {
    callback: Box<TimerCallback>,
    next_call_time: time::SystemTime,
    millis: u64,
    once: bool,
    id: TimerId,
}

// derive(Debug) didn't work because of `callback: Box<TimerCallback>`
impl Debug for TimerData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TimerData {{ {} }}", self.id)
    }
}

#[derive(Debug, Default)]
pub struct ScheduleState {
    id: TimerId,
    timers: Vec<TimerData>,
    to_be_destroyed: Vec<TimerId>,
}

impl ScheduleState {
    pub fn create(&mut self, callback: Box<TimerCallback>, millis: u64, once: bool) -> TimerId {
        let id = {
            self.id += 1;
            self.id
        };

        log!("creating timer with id: {id}");

        let next_call_time = time::SystemTime::now() + time::Duration::from_millis(millis);

        self.timers.push(TimerData {
            callback,
            next_call_time,
            millis,
            once,
            id,
        });

        id
    }

    pub fn add_to_be_destroyed(&mut self, id: TimerId) {
        self.to_be_destroyed.push(id);
    }
}

#[derive(Debug, Default)]
pub struct TimerManager {
    timers: Vec<TimerData>,
}

impl TimerManager {
    pub fn process_timers(&mut self, mut schedule: RefMut<ScheduleState>) {
        self.timers.append(&mut schedule.timers);

        if !schedule.to_be_destroyed.is_empty() {
            for idx in self.get_to_be_destroyed(&schedule) {
                self.timers.swap_remove(idx);
            }
            schedule.to_be_destroyed.clear();
        }

        drop(schedule); // unborrow ScheduleState

        let mut indexes_to_remove: Vec<usize> = vec![];
        let now = time::SystemTime::now();

        for (idx, timer) in self.timers.iter_mut().enumerate().rev() {
            if now >= timer.next_call_time {
                (timer.callback)();
                log!("timer callback called successfully");

                if timer.once {
                    indexes_to_remove.push(idx);
                    continue;
                }
                timer.next_call_time =
                    time::SystemTime::now() + time::Duration::from_millis(timer.millis);
            }
        }

        for idx in indexes_to_remove {
            self.timers.swap_remove(idx);
        }
    }

    fn get_to_be_destroyed(&self, schedule: &RefMut<ScheduleState>) -> Vec<usize> {
        let mut indexes: Vec<usize> = schedule
            .to_be_destroyed
            .iter()
            .filter_map(|id| {
                let timer = self
                    .timers
                    .iter()
                    .enumerate()
                    .find(|(_, t)| t.id == *id)
                    .map(|(idx, _)| idx);

                if timer.is_none() {
                    log_error!(
                        "Failed to destroy timer with id: {id} (it was probably already removed)"
                    );
                } else {
                    log!("destroying timer with id: {id}");
                }
                timer
            })
            .collect();
        indexes.sort_unstable_by(|a, b| b.cmp(a));
        indexes
    }
}

pub fn create_timer(callback: Box<dyn FnMut() + 'static>, millis: u64, once: bool) -> Timer {
    let id = Resource::with_timer_schedule_mut(|mut t, _| t.create(callback, millis, once));
    Timer::new(id)
}

// pub fn remove_timer(id: TimerId) {
//     Resource::with(|v| {
//         let schedule = v.timer_schedule.try_borrow_mut();
//         let Ok(mut schedule) = schedule else {
//             bail!("Failed to mutably borrow timer schedule");
//         };
//         schedule.add_to_be_destroyed(id);
//         Ok(())
//     })
// }

#[derive(Debug)]
pub struct Timer {
    id: Option<TimerId>,
}

impl Timer {
    pub(crate) fn new(id: TimerId) -> Self {
        Self { id: Some(id) }
    }

    // pub fn destroy(&mut self) {
    //     let Some(id) = self.id else {
    //         log_error("Already destroyed");
    //         return;
    //         // bail!("Already destroyed")
    //     };

    //     let res = remove_timer(id);
    //     if let Ok(()) = res {
    //         self.id = None;
    //     }
    //     res
    // }

    pub fn id(&self) -> Option<TimerId> {
        self.id
    }
}
