use std::{time::Duration, rc::Rc, cell::RefCell};

use altv::{event::EventHandler, SharedVehicle, dbg};

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    let mut api = altv::Api::new();
    let mut vehicles = api.vehicles;

    let mut get_vehicle_fuel_level = move |id: u32| {
        altv::spawn_async(async move {
            altv::wait(Duration::from_millis(500)).await;

            // uncomment this and it will not compile
            // let vehicle = vehicles.get_by_id(id).unwrap();
            // vehicle.fuel_level();
        })
        .unwrap();
    };

    altv::event::add_handler(EventHandler::EnteredVehicle(Box::new(move |cx| {
        dbg!(cx);
        dbg!(cx.vehicle.fuel_level());
        dbg!(cx.vehicle.set_fuel_level(1.0));
        dbg!(cx.vehicle.fuel_level());

        // ------------------------------------------------------------------------
        // TODO: investigate how this stuff will affect "js way of altv coding"

        // does not compile:
        // let veh = &cx.vehicle;
        // altv::spawn_async(async move {
        //     altv::wait(Duration::from_millis(500)).await;
        //     veh.fuel_level();
        // })
        // .unwrap();

        // tho its possible to workaround it using id
        // but its unsafe because ids are NOT unique and reused
        // maybe expose ptr for this use case?
        get_vehicle_fuel_level(cx.vehicle.id());
    })));
}
