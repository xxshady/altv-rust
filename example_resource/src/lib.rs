use alt::Entity;

#[alt::main(crate_name = "alt")]
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

    // alt::events::on_server_started(|controller| {
    //     alt::log_warn!("example resource on_server_started controller: {controller:?}");
    // });

    // alt::events::on_player_connect(|alt::events::PlayerConnectController { player }| {
    //     let player = player.borrow();
    //     let id = player.id().unwrap();

    //     alt::log!(
    //         "example resource on_player_connect name: {} id: {}",
    //         player.name().unwrap(),
    //         id
    //     );

    //     dbg!(alt::Player::get_by_id(id));
    //     let result = id.checked_sub(1);
    //     if let Some(id) = result {
    //         dbg!(alt::Player::get_by_id(dbg!(id)));
    //     } else {
    //         alt::log_error!("failed to sub player id");
    //     }
    // });

    // alt::set_timeout(|| println!("its timeout"), 300);

    // let vehicle = alt::Vehicle::new(alt::hash("sultan"), 0.into(), 0.into()).unwrap();
    // dbg!(&vehicle);

    // let id = vehicle.borrow().id().unwrap();
    // dbg!(&id);

    // dbg!(alt::Vehicle::get_by_id(id));

    // vehicle.borrow_mut().destroy().unwrap();

    // dbg!(alt::Vehicle::get_by_id(id));

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

    fn recurse_set_timeout() {
        alt::set_timeout(
            || {
                alt::log!("set_timeout ~gl~recursion");
                recurse_set_timeout();
            },
            500,
        )
    }
    recurse_set_timeout();
}
