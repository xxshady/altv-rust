use std::time::Duration;

use altv::{SharedVehicle, WorldObject, ClientWorldObject, Vector3};

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    let mut api = altv::Api::new();

    // let value = 123;
    // panic!("startup panic example {value}");

    // altv::set_timeout(
    //     || {
    //         panic!("pppppppanic");
    //     },
    //     500,
    // );

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
        true,
        100,
    );
    altv::dbg!(local_vehicle.fuel_level());

    altv::dbg!(local_vehicle.pos());
    local_vehicle.set_pos(Vector3 {
        x: 5.0,
        y: 5.0,
        z: 75.0,
    });
    altv::dbg!(local_vehicle.pos());

    // altv::dbg!(api.local_vehicles.all());

    altv::set_interval(
        move || {
            let current = altv::dbg!(local_vehicle.dimension());
            local_vehicle.set_dimension((!(current != 0)) as i32);
        },
        1000,
    );

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

    altv::spawn_async(async {
        altv::wait(Duration::from_millis(500)).await;
    })
    .unwrap();
}
