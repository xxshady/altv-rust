use std::{
    future::{Future, poll_fn},
    task::Poll,
    time::{Duration, Instant},
    rc::Rc,
    cell::Cell,
};
use anyhow::anyhow;

use crate::{timers::set_timeout, VoidResult};

pub fn wait(duration: Duration) -> impl Future {
    let dest = Instant::now() + duration;
    let mut timer_was_set = false;

    poll_fn(move |cx| {
        if Instant::now() >= dest {
            return Poll::Ready(());
        }
        if timer_was_set {
            return Poll::Pending;
        }
        timer_was_set = true;

        let waker = cx.waker().clone();
        set_timeout(
            Box::new(|| {
                waker.wake();
            }),
            duration,
        );

        Poll::Pending
    })
}

pub fn wait_for(
    mut callback: impl FnMut() -> bool,
    timeout: Duration,
) -> impl Future<Output = VoidResult> {
    let now = Instant::now();
    let expecting_poll = Rc::new(Cell::new(true));

    poll_fn(move |cx| {
        if !expecting_poll.get() {
            return Poll::Pending;
        }
        expecting_poll.set(false);

        if callback() {
            return Poll::Ready(Ok(()));
        }
        if now.elapsed() >= timeout {
            return Poll::Ready(Err(anyhow!("Failed to wait for callback")));
        }

        let waker = cx.waker().clone();
        let expecting_poll = expecting_poll.clone();
        set_timeout(
            Box::new(move || {
                expecting_poll.set(true);
                waker.wake();
            }),
            Duration::ZERO,
        );
        return Poll::Pending;
    })
}
