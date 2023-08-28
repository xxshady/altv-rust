#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    let mut api = altv::Api::new();

    let local_vehicle: altv::LocalVehicle = api.local_vehicles.create(
        altv::hash("sultanrs"),
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

    altv::set_timeout(
        move || {
            altv::log!("destroying");
            api.local_vehicles.destroy(local_vehicle);
        },
        2000,
    );
}
