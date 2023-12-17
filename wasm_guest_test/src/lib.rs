use std::{time::Duration, rc::Rc, cell::RefCell};
use altv::{event::EventHandler, dbg, natives, log, RemoteBaseObject};

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    // TODO: retrieve base objects that were created before this resource started

    let v = altv::LocalVehicle::new_streamed(
        altv::hash("driftremus"),
        0,
        3.,
        0.,
        71.0,
        0.,
        0.,
        0.,
        10,
        |s| {
            log!("spawn {s:?}");
            natives::set_vehicle_colours(s, 10, 10);
        },
        || {
            log!("despawn");
        },
    )
    .unwrap();

    std::mem::forget(v);

    // let c: Rc<RefCell<Option<altv::event::EventController>>> = Rc::default();
    // let c_ = c.clone();
    // c.borrow_mut()
    //     .replace(altv::event::add_handler(EventHandler::GameEntityCreate(
    //         Box::new(move |ctx| {
    //             log!("create entity: {:?}", ctx.entity);
    //             c_.take().unwrap().destroy();
    //         }),
    //     )));
}
