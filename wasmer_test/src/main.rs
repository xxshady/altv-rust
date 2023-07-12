mod resource;
mod timers;

wai_bindgen_rust::import!("api.wai");

wai_bindgen_rust::export!("exports.wai");

struct Exports;

impl exports::Exports for Exports {
    fn every_tick() {
        Resource::with_timers_mut(|mut timers, resource| {
            timers.process_timers(resource.timer_schedule.borrow_mut());
        });
    }
}

#[macro_export]
macro_rules! __log_error {
    ($($arg:tt)*) => {{
        $crate::api::log_error(&format!($($arg)*))
    }}
}

pub use __log_error as log_error;

use crate::{resource::Resource, timers::create_timer};

fn main() {
    Resource::init();

    create_timer(
        Box::new(|| {
            println!("timer");
        }),
        1500,
        false,
    );

    let id = api::create_marker(0, 0., 0., 74.);
    println!("created marker id: {id:?}");
}
