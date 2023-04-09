use std::{cell::RefCell, rc::Rc};

pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    // TODO: fix backtrace panic
    std::env::set_var("RUST_BACKTRACE", "full");

    alt::Vehicle::new("sultan", 0, 0).unwrap();
    alt::Vehicle::new("sultan", 0, 0).unwrap();
    alt::Vehicle::new("sultan", 0, 0).unwrap();

    // assert_eq!(alt::Vehicle::all().len(), 3);

    // alt::Vehicle::all()
    //     .iter()
    //     .try_for_each(|v| v.destroy())
    //     .unwrap();

    // dbg!(alt::Vehicle::all());
    // assert_eq!(alt::Vehicle::all().len(), 0);

    alt::set_timeout(
        || {
            dbg!(alt::Vehicle::all());
            alt::events::emit!("destroy-vehs");
            Ok(())
        },
        100,
    );

    alt::set_timeout(
        || {
            dbg!(alt::Vehicle::all());
            Ok(())
        },
        200,
    );
}
