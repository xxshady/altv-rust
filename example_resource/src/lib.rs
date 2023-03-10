use alt::Entity;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    // alt::set_timeout(
    //     || {
    //         alt::log!("example set_timeout");
    //         alt::set_timeout(
    //             || {
    //                 alt::log!("example nested set_timeout");
    //             },
    //             500,
    //         )
    //     },
    //     500,
    // );

    // timers unloading on resource stop
    // alt::log!("start");
    // for i in 0..100 {
    //     let mut vec = vec![];

    //     for _ in 0..50_000 {
    //         vec.push(Box::new(u64::MAX));
    //     }

    //     alt::set_interval(
    //         move || {
    //             alt::log!("interval {} vec: {:?}", i, vec);
    //         },
    //         100000u64,
    //     );
    // }
    // alt::log!("~gl~end");

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

    alt::events::on_player_connect(|alt::events::PlayerConnectController { player }| {
        {
            let player = player.borrow();
            let id = player.id().unwrap();

            alt::log!(
                "example resource on_player_connect name: {} id: {}",
                player.name().unwrap(),
                id
            );
        }

        alt::set_interval(
            move || {
                let vehicle = alt::Vehicle::new(alt::hash("sultan2"), 0.into(), 0.into()).unwrap();
                alt::events::emit_client!(
                    "test",
                    player.clone(),
                    "test",
                    123u64,
                    alt::events::list![vehicle].unwrap()
                )
                .unwrap();

                alt::events::emit_client!("test", player.clone()).unwrap();

                alt::events::emit_all_clients!("test", "emit all", player.clone()).unwrap();

                alt::events::emit_some_clients!(
                    "test",
                    vec![player.clone()],
                    "emit some",
                    player.clone()
                )
                .unwrap();
            },
            1000,
        );
    });

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

    // TODO: check resource restart with created vehicle here:
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

            alt::events::on("test".to_string(), |args| {
                alt::log!("test event args: {:?}", args);
                // args.get_dict_at(4)?;
                Ok(())
            });

            for _ in 0..1 {
                let mut str = String::from("");
                for _ in 1..100 {
                    let a: i32 = rand::random();
                    str += a.to_string().as_str();
                }

                alt::set_timeout(
                    move || {
                        let vehicle =
                            alt::Vehicle::new(alt::hash("sultan2"), 0.into(), 0.into()).unwrap();

                        // vehicle.borrow_mut().destroy().unwrap();

                        alt::events::emit!("js-destroy-veh", vehicle.clone()).unwrap();

                        alt::set_timeout(
                            move || {
                                alt::events::emit!(
                                    "test",
                                    69i64,
                                    true,
                                    "string",
                                    // TODO: make this shit more user-friendly if dict! or list! result is passed directly without unwrapping it
                                    alt::events::dict! {
                                        "kek" => 123i64,
                                        // "vehicle" => vehicle.clone(),
                                    }
                                    .unwrap()
                                )
                                .unwrap();
                            },
                            500,
                        )
                    },
                    500,
                );

                // vehicle.borrow_mut().destroy().unwrap();
            }

            alt::log!("~gl~done");
        },
        100,
    );

    // // should raise warning
    // alt::events::emit!("test2");
}
