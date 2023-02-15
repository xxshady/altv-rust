#[alt::res_main]
#[no_mangle]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    // let mut i = 0;
    // alt::set_interval(
    //     move || {
    //         i += 1;
    //         alt::log!("test interval i: {i}");
    //     },
    //     1000,
    // );

    // let mut i = 0;
    // alt::events::on_server_started(move |controller| {
    //     i += 1;
    //     alt::log!("example resource ServerStarted controller: {controller:?} i: {i:?}");
    // });

    alt::events::on_server_started(|_| {
        alt::log!("example resource on_server_started!!!!!!");
    });

    alt::events::on_player_connect(|data| {
        alt::log!("example resource on_player_connect!!!!!!");
        dbg!(data.player.borrow().name());
    });

    // alt::set_timeout(|| println!("its timeout"), 300);

    // let vehicle = alt::Vehicle::new(alt::hash("sultan"), 0.into(), 0.into()).unwrap();
    // dbg!(&vehicle);
    // vehicle.borrow_mut().destroy().unwrap();
    // let mut veh = vehicle.try_lock().unwrap();

    // dbg!(veh.id().unwrap());
    // dbg!(veh.set_secondary_color(10).unwrap());

    // // dbg!(veh.destroy());

    // drop(veh);

    // let test_veh_get_by_id = |id: alt::EntityId| {
    //     if let Some(v) = alt::Vehicle::get_by_id(id) {
    //         alt::log_warn!(
    //             "get_by_id veh id: {id} get_secondary_color: {:?}",
    //             v.try_lock().unwrap().get_secondary_color()
    //         );
    //     } else {
    //         alt::log_warn!("get_by_id veh not found id: {id}");
    //     }
    // };
    // test_veh_get_by_id(0);

    // alt::set_timeout(
    //     move || {
    //         alt::log_warn!("timeout rust test");
    //         let mut veh = vehicle.try_lock().unwrap();
    //         dbg!(veh.get_secondary_color());

    //         // drop(veh);
    //         // alt::Vehicle::destroy_vehicle(vehicle);
    //         // veh.destroy().unwrap_or_else(|e| panic!("error: {e}"));
    //         // dbg!(veh.get_secondary_color().unwrap());
    //         veh.destroy();
    //         drop(veh);
    //         test_veh_get_by_id(1);
    //         test_veh_get_by_id(0);
    //     },
    //     1000,
    // );

    // fn recurse_set_timeout() {
    //     alt::set_timeout(
    //         || {
    //             alt::log!("set_timeout ~gl~recursion");
    //             recurse_set_timeout();
    //         },
    //         500,
    //     )
    // }
    // recurse_set_timeout();
}
