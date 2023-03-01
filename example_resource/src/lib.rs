use alt::{
    events::{ConsoleCommandController, PlayerDisconnectController},
    Entity,
};

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

    //     // let args = cxx::UniquePtr::new();

    //     // alt::__test_trigger_client_event(player.ptr(), &event_name, args)

    //     // dbg!(alt::Player::get_by_id(id));
    //     // let result = id.checked_sub(1);
    //     // if let Some(id) = result {
    //     //     dbg!(alt::Player::get_by_id(dbg!(id)));
    //     // } else {
    //     //     alt::log_error!("failed to sub player id");
    //     // }
    // });

    // alt::events::on_player_disconnect(|PlayerDisconnectController { player, reason }| {
    //     let player = player.borrow();
    //     alt::log_warn!(
    //         "on_player_disconnect player: {} [{}], reason: {}",
    //         player.name().unwrap(),
    //         player.id().unwrap(),
    //         reason
    //     )
    // });

    // alt::events::on_console_command(|ConsoleCommandController { name, args }| {
    //     alt::log_warn!("on_console_command name: {name:?} args: {args:?}");
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

    alt::set_timeout(
        || {
            alt::log!("~cl~start");

            let max_uint = u64::MAX;
            let max_uint2 = max_uint;
            let max_int = i64::MAX;
            let max_int2 = i64::MAX;

            alt::events::on("test".to_string(), |_| {
                // // alt::log!("received test args: {args:?}");

                // let bool = args.get_bool_at(0)?;
                // let f64 = args.get_f64_at(1)?;
                // let string = args.get_string_at(2)?;

                // // alt::log!("bool: {bool:?}");
                // // alt::log!("f64: {f64:?}");
                // // alt::log!("string: {string:?}");

                // assert_eq!(max_uint2, *args.get_u64_at(4)?);
                // assert_eq!(max_int2, *args.get_i64_at(5)?);

                // let list = args.get_list_at(6)?;
                // let list_string = list.get_string_at(0)?;
                // // alt::log!("list: {list:?} list_string: {list_string:?}");

                Ok(())
            });

            for i in 0..1 {
                let mut str = String::from("");
                for _ in 1..100 {
                    let a: i32 = rand::random();
                    str += a.to_string().as_str();
                }

                alt::events::emit!(
                    "test",
                    true,
                    0.5,
                    str.as_str(),
                    alt::events::None,
                    max_uint,
                    max_int,
                    alt::events::nested_args!["aa", "b"]
                );
            }

            alt::log!("~gl~done");
        },
        3000,
    );

    // // should raise warning
    // alt::events::emit!("test2");
}
