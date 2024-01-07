use std::{time::Duration, rc::Rc, cell::RefCell};
use altv::{event::EventHandler, dbg, natives, log, RemoteBaseObject, SpawnedVehicle};

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    altv::spawn_async(async {
        let veh =
            altv::LocalVehicle::new_static(altv::hash("sultan2"), 0, 3., 0., 71.0, 0., 0., 0.)
                .await
                .unwrap();

        for i in 1..=15 {
            altv::wait(Duration::from_secs(1)).await;
            let alpha = ((1.0 - ((i as f32) / 15.0)) * 255.0) as i32;
            natives::set_entity_alpha(&veh, alpha, false);

            altv::log!("waiting {i} alpha: {alpha}...");
        }
    })
    .unwrap();

    // let veh: Rc<RefCell<Option<altv::LocalVehicleStreamed>>> = Rc::default();
    // let veh_ = veh.clone();
    // let v = altv::LocalVehicle::new_streamed(
    //     altv::hash("driftremus"),
    //     0,
    //     3.,
    //     0.,
    //     71.0,
    //     0.,
    //     0.,
    //     0.,
    //     10,
    //     move |s| {
    //         log!("spawn {s:?}");
    //         natives::set_vehicle_colours(s, 20, 5);
    //         // natives::set_entity_alpha(s, 100, false);

    //         // let veh_ = veh_.clone();
    //         // altv::set_timeout(
    //         //     move || {
    //         //         drop(veh_.borrow_mut().take().unwrap());
    //         //     },
    //         //     Duration::from_secs(2),
    //         // );
    //     },
    //     move || {
    //         log!("despawn");
    //         drop(veh_.borrow_mut().take().unwrap());
    //     },
    // )
    // .unwrap();

    // veh.borrow_mut().replace(v);

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
