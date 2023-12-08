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
    })));
}
