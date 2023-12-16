use std::{time::Duration, rc::Rc, cell::RefCell};

use altv::{event::EventHandler, dbg, natives, log};

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    // TODO: retrieve base objects that were created before this resource started

    let veh = Rc::new(RefCell::new(None));

    let v = veh.clone();
    altv::spawn_async(async move {
        let veh_ =
            altv::LocalVehicle::new_static(altv::hash("driftremus"), 0, 0., 0., 72., 0., 0., 0.)
                .await?;

        let s = veh_.script_id();
        natives::set_vehicle_custom_primary_colour(s, 255, 0, 0);

        v.borrow_mut().replace(veh_);

        Ok(())
    })
    .unwrap();

    altv::event::add_handler(EventHandler::EnteredVehicle(Box::new(move |cx| {
        let altv::AnyVehicle::LocalVehicle(ref veh_ref) = cx.vehicle else {
            unreachable!();
        };
        dbg!(veh_ref.id());

        {
            let veh = veh.borrow();
            let veh = veh.as_ref().unwrap();

            if veh_ref != veh {
                log!("entered other local vehicle");
                return;
            }
        }

        let veh = veh.borrow_mut().take().unwrap();
        let s = veh.script_id();
        natives::set_vehicle_mod_kit(s, 0);
        natives::set_vehicle_custom_primary_colour(s, 0, 255, 0);

        altv::spawn_async(async move {
            let s = veh.script_id();
            for i in 1..=10 {
                natives::set_vehicle_mod(s, 0, i, false);
                altv::wait(Duration::from_millis(500)).await;
            }
            std::mem::forget(veh);
        })
        .unwrap();
    })));
}
