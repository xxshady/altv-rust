pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    alt::set_timeout(
        move || {
            let vehicle = alt::Vehicle::new("sultan2", 3, 0).unwrap();
            let id = vehicle.borrow().id().unwrap();
            alt::log!("~gl~id: {id}");
            let valid = vehicle.borrow().valid();
            alt::log!("~gl~valid: {valid}");
            alt::log!("~gl~created entity {vehicle:?}");

            alt::log!("pos {:?}", vehicle.try_borrow()?.pos()?);

            alt::set_timeout(
                move || {
                    let vehicle = dbg!(alt::Vehicle::get_by_id(id))?;

                    vehicle.borrow_mut().destroy();
                    alt::events::emit!("destroy-veh", vehicle.clone())?;

                    alt::set_timeout(
                        move || {
                            dbg!(alt::Vehicle::get_by_id(id));
                            dbg!(vehicle.borrow().id());
                            Ok(())
                        },
                        300,
                    );

                    Ok(())
                },
                300,
            );
            Ok(())
        },
        300,
    );
}
