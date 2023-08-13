mod resource;
mod timers;

#[macro_export]
macro_rules! __log_error {
    ($($arg:tt)*) => {
        // $crate::api::log_error(&format!($($arg)*))
        todo!()
    };
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
}
