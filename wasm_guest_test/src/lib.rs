#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    let mut api = altv::Api::new();

    altv::dbg!(api.local_vehicles.all());

    let local_vehicle: altv::LocalVehicle = api.local_vehicles.create(
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

    altv::dbg!(api.local_vehicles.all());

    // println!("kek");
    dbg!("kek");

    // altv::set_timeout(
    //     move || {
    //         altv::dbg!(local_vehicle.fuel_level());
    //         altv::log!("destroying");

    //         api.local_vehicles.destroy(local_vehicle);

    //         altv::dbg!(api.local_vehicles.all());
    //     },
    //     2000,
    // );
}
