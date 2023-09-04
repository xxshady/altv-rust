#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    let mut api = altv::Api::new();

    altv::set_timeout(
        || {
            panic!("pppppppanic");
        },
        500,
    );

    altv::dbg!(api.local_vehicles.all());

    let local_vehicle = api.local_vehicles.create(
        altv::hash("sultan"),
        0,
        0.0,
        5.0,
        72.0,
        0.0,
        0.0,
        0.0,
        false,
        0,
    );
    altv::dbg!(local_vehicle.fuel_level());

    // altv::dbg!(api.local_vehicles.all());

    // altv::set_interval(
    //     move || {
    //         altv::dbg!(local_vehicle.id());
    //     },
    //     1000,
    // );

    // altv::set_timeout(
    //     move || {
    //         local_vehicle.set_fuel_level(30.0);
    //         altv::dbg!(local_vehicle.fuel_level());
    //         altv::log!("destroying");

    //         api.local_vehicles.destroy(local_vehicle);

    //         altv::dbg!(api.local_vehicles.all());
    //     },
    //     2500,
    // );
}
