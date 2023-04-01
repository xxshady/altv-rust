use std::borrow::Borrow;

pub use alt::prelude::*;
use autocxx::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    alt::set_timeout(
        move || {
            let colshape = alt::ColShape::new_circle(alt::Vector2::new(2., 1.), 5.);
            alt::log!("~gl~created colshape {colshape:?}");

            alt::log!("pos {:?}", colshape.try_borrow()?.pos()?);

            alt::set_timeout(
                move || {
                    colshape.borrow_mut().destroy();
                    Ok(())
                },
                300,
            );
            Ok(())
        },
        300,
    );
}
