#![deny(clippy::dbg_macro, reason = "use `altv::dbg!` instead")]
#![deny(clippy::print_stdout, reason = "use `log_info!` instead")]
#![deny(clippy::print_stderr, reason = "use `log_error!` instead")]

use async_executor::EXECUTOR_INSTANCE;
use timers::{TIMER_MANAGER_INSTANCE, TIMER_SCHEDULE_INSTANCE};
use wasm_bindgen::prelude::*;

mod altv_events;
mod script_events;
mod async_executor;
mod logging;
mod wasm_imports;
mod id_provider;
mod panic_handler;
mod any_void_result;
mod any_error;
mod helpers;
mod timers;
mod wait;
mod base_objects;
mod vector;
mod joaat;

use logging::log_info;

#[wasm_bindgen]
pub fn main() {
  panic_handler::init();

  log_info!("start");

  base_objects::manager::init();
}

#[wasm_bindgen]
pub fn on_every_tick() {
  EXECUTOR_INSTANCE.with_borrow_mut(|executor| {
    executor.run();
  });
  TIMER_MANAGER_INSTANCE.with_borrow_mut(|timers| {
    TIMER_SCHEDULE_INSTANCE.with(|schedule| timers.process_timers(schedule.borrow_mut()));
  });
}
