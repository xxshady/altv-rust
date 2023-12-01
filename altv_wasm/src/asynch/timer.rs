use std::{
    future::{Future, poll_fn},
    pin::Pin,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use anyhow::anyhow;

use crate::{timers::set_timeout, VoidResult};

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

pub fn wait_for(
    mut callback: impl FnMut() -> bool,
    timeout: Duration,
) -> impl Future<Output = VoidResult> {
    let now = Instant::now();

    poll_fn(move |cx| {
        if callback() {
            return Poll::Ready(Ok(()));
        }
        if now.elapsed() >= timeout {
            return Poll::Ready(Err(anyhow!("Failed to wait for callback")));
        }

        let waker = cx.waker().clone();
        set_timeout(
            Box::new(move || {
                waker.wake();
            }),
            Duration::ZERO,
        );
        return Poll::Pending;
    })
}
