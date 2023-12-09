use std::time::Duration;

use altv::{event::EventHandler, dbg};

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    // TODO: retrieve base objects that were created before this resource started

    let veh = altv::LocalVehicle::new(altv::hash("sultan"), 0, 0., 0., 72., 0., 0., 0., false, 0);
    let mut veh = Some(veh);

    altv::event::add_handler(EventHandler::EnteredVehicle(Box::new(move |cx| {
        let all_count = altv::LocalVehicle::read_all(|all| {
            dbg!(all);
            all.len()
        });
        dbg!(all_count);

        dbg!(veh.as_ref().unwrap().id());
        veh.take().unwrap(); // drop it

        let altv::AnyVehicle::Local(ref veh) = cx.vehicle else {
            unreachable!();
        };
        dbg!(veh.id());

        let all_count = altv::LocalVehicle::read_all(|all| {
            dbg!(all);
            all.len()
        });
        dbg!(all_count);

        altv::set_timeout(
            || {
                let all_count = altv::LocalVehicle::read_all(|all| {
                    dbg!(all);
                    all.len()
                });
                dbg!(all_count);
            },
            Duration::from_millis(500),
        );
    })));

    altv::spawn_async(async {
        loop {
            altv::Vehicle::read_all(|all| {
                altv::log!("all vehicles count: {}", all.len());
                if all.len() == 0 {
                    return;
                }
                let [veh] = all else {
                    unreachable!();
                };
                altv::Vehicle::read_by_id(veh.id(), |v| {
                    dbg!(v.id() == veh.id());

                    altv::Vehicle::read_all(|all| {
                        dbg!(all.iter().map(|v| v.id()).collect::<Vec<_>>());
                    });

                    // altv::Vehicle::read_by_id(v.id(), |vv| {
                    //     dbg!(vv.id() == veh.id());
                    // });
                });
            });
            altv::wait(Duration::from_secs(1)).await;
        }
    })
    .unwrap();
}
