use std::{time::Duration, rc::Rc, cell::RefCell};
use altv::{event::EventHandler, dbg, natives, log, RemoteBaseObject};

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    // TODO: retrieve base objects that were created before this resource started

    altv::set_interval(
        || {
            altv::Vehicle::read_all_spawned(|vehicles| {
                let [first, ..] = vehicles else {
                    log!("no first");
                    return;
                };

                altv::Vehicle::read_by_remote_id(first.remote_id(), |veh| {
                    dbg!(veh.script_id());
                });

                let vehs = vehicles.iter().map(|v| v.script_id()).collect::<Vec<_>>();
                dbg!(vehs);
            });
        },
        Duration::from_secs(1),
    );
}
