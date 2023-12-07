use std::{time::Duration, rc::Rc, cell::RefCell};

use altv::{event::EventHandler, SharedVehicle, dbg};

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    let mut api = altv::Api::new();

    let veh_ =
        api.local_vehicles
            .create(altv::hash("sultan"), 0, 0., 0., 72., 0., 0., 0., false, 0);
    let mut veh_ = Some(veh_);

    altv::event::add_handler(EventHandler::EnteredVehicle(Box::new(move |cx| {
        dbg!(cx);

        // TODO: this must not be allowed
        api.local_vehicles.destroy(veh_.take().unwrap());

        let altv::AnyVehicle::Local(ref get_veh) = cx.vehicle else {
            altv::log!("ignoring server vehicle");
            return;
        };

        // TODO: do not allow multiple calls:
        // note: i tried FnOnce but it did not work with current implementation
        // because event context is passed by shared reference
        get_veh(&api.local_vehicles);
        get_veh(&api.local_vehicles);

        let veh = get_veh(&api.local_vehicles);

        // cannot be called while veh is in use
        // api.local_vehicles.destroy(veh_.take().unwrap());

        dbg!(veh.fuel_level());
        dbg!(veh.set_fuel_level(1.0));
        dbg!(veh.fuel_level());

        // veh is not used anymore so we can safely destroy it
        api.local_vehicles.destroy(veh_.take().unwrap());

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
        // get_vehicle_fuel_level(veh.id());
    })));
}
