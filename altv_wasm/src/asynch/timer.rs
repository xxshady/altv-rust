use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};

use crate::timers::set_timeout;

pub(crate) struct Timer {
    dest: Instant,
    duration: Duration,
    timer_was_set: bool,
}

impl Timer {
    pub(crate) fn new(duration: Duration) -> Self {
        Timer {
            dest: Instant::now() + duration,
            duration,
            timer_was_set: false,
        }
    }
}

impl Future for Timer {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.dest {
            return Poll::Ready(());
        }

        // waker could be shared between different tasks (futures)
        if !self.timer_was_set {
            self.timer_was_set = true;

            let waker = cx.waker().clone();
            set_timeout(
                Box::new(move || {
                    waker.wake();
                }),
                self.duration,
            );
        }

        Poll::Pending
    }
}

pub fn wait(duration: Duration) -> impl Future {
    Timer::new(duration)
}
