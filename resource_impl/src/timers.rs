use std::{cell::RefMut, fmt::Debug};

pub type TimerId = u32;

pub type TimerCallback = dyn FnMut() + 'static;

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

#[derive(Debug)]
pub struct ScheduleState {
    id: TimerId,
    timers: Vec<Timer>,
}

impl ScheduleState {
    pub fn new() -> Self {
        Self {
            id: 0,
            timers: vec![],
        }
    }
}

#[derive(Debug)]
pub struct TimerManager {
    timers: Vec<Timer>,
}

impl TimerManager {
    pub fn new() -> Self {
        Self { timers: vec![] }
    }

    pub fn process_timers(&mut self, mut schedule: RefMut<ScheduleState>) {
        self.timers.append(&mut schedule.timers);
        drop(schedule); // unborrow ScheduleState

        let now = std::time::SystemTime::now();
        let mut indexes_to_remove: Vec<usize> = vec![];

        for (idx, timer) in self.timers.iter_mut().enumerate() {
            if now >= timer.next_call_time {
                (timer.callback)();
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

pub fn create(
    mut state: RefMut<ScheduleState>,
    callback: Box<TimerCallback>,
    millis: u64,
    once: bool,
) -> TimerId {
    let id = {
        state.id += 1;
        state.id
    };

    logger::debug!("creating timer with id: {id}");

    let next_call_time = std::time::SystemTime::now() + std::time::Duration::from_millis(millis);

    state.timers.push(Timer {
        callback,
        next_call_time,
        millis,
        once,
    });

    id
}
