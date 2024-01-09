use std::{time::Duration, rc::Rc, cell::RefCell};
use altv::{event::EventHandler, dbg, natives, log, RemoteBaseObject, SpawnedVehicle, WorldObject};

#[no_mangle]
extern "C" fn main() {
    altv::spawn_async(async {
        let veh =
            altv::LocalVehicle::new_static(altv::hash("sultan2"), 0, 5., 0., 71.0, 0., 0., 0.)
                .await?;
        veh.pos();
        natives::set_entity_alpha(&veh, 100, true);

        let _seats = veh.seat_count();

        let veh = altv::LocalVehicle::new_streamed(
            altv::hash("sultan2"),
            0,
            5.,
            0.,
            71.0,
            0.,
            0.,
            0.,
            10,
            |veh| {
                let _seats = veh.seat_count();
                veh.pos();
            },
            || {},
        )?;
        veh.pos();
        std::mem::forget(veh);

        // for i in 1..=15 {
        //     altv::wait(Duration::from_secs(1)).await;
        //     let alpha = ((1.0 - ((i as f32) / 15.0)) * 255.0) as i32;
        //     natives::set_entity_alpha(&veh, alpha, true);

        //     altv::log!("waiting {i} alpha: {alpha}...");
        // }

        Ok(())
    })
    .unwrap();
}
