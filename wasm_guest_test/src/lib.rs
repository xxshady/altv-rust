#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~main");

    let mut i = 0;
    altv::set_interval(
        move || {
            i += 1;
            altv::dbg!(i);
            Ok(())
        },
        3000,
    );

    let mut vehicles = altv::vehicles();

    let vehicle = vehicles.new(
        altv::hash("sultan2"),
        0,
        0.0,
        0.0,
        72.0,
        0.0,
        0.0,
        0.0,
        false,
        0,
    );

    altv::set_timeout(
        move || {
            dbg!(&vehicle);

            dbg!(vehicle.fuel_level());

            vehicle.set_fuel_level(0.3);

            dbg!(vehicle.fuel_level());

            // vehicles.destroy(vehicle);
        },
        2000,
    );
}
