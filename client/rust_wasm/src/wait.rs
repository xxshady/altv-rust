use crate::{any_error::AnyError, base_objects::scope::Scope, helpers::net_time, timers::set_timeout};
use std::{
  cell::Cell,
  future::{poll_fn, Future},
  ops::Deref,
  rc::Rc,
  task::Poll,
  time::Duration,
};

pub fn wait(duration: Duration) -> impl Future {
  let dest = net_time() + duration;
  let mut timer_was_set = false;

  poll_fn(move |cx| {
    if timer_was_set {
      if net_time() >= dest {
        return Poll::Ready(());
      }

      return Poll::Pending;
    }
    timer_was_set = true;

    let waker = cx.waker().clone();
    set_timeout(
      |_| {
        waker.wake();
      },
      duration,
    );

    Poll::Pending
  })
}

pub struct WaitForContext(());
impl Scope for WaitForContext {}

#[must_use]
pub async fn wait_for(
  mut callback: impl FnMut(&WaitForContext) -> bool,
  timeout: Duration,
) -> bool {
  let timeout_dest = net_time() + timeout;

  loop {
    if callback(&WaitForContext(())) {
      return true;
    }

    if net_time() >= timeout_dest {
      return false;
    }

    next_tick().await;
  }
}

// TODO: probably needs refactor if we need better perf in loops (see wait_for)
pub async fn next_tick() {
  wait(Duration::ZERO).await;
}
