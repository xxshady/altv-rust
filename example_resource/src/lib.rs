pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    // alt::set_timeout(
    //     move || {
    //         let vehicle = alt::Vehicle::new("sultan2", 3, 0).unwrap();
    //         let id = vehicle.borrow().id().unwrap();
    //         alt::log!("~gl~id: {id}");
    //         let valid = vehicle.borrow().valid();
    //         alt::log!("~gl~valid: {valid}");
    //         alt::log!("~gl~created entity {vehicle:?}");

    //         alt::log!("pos {:?}", vehicle.try_borrow()?.pos()?);

    //         alt::set_timeout(
    //             move || {
    //                 let vehicle = dbg!(alt::Vehicle::get_by_id(id))?;

    //                 vehicle.borrow_mut().destroy();
    //                 alt::events::emit!("destroy-veh", vehicle.clone())?;

    //                 alt::set_timeout(
    //                     move || {
    //                         dbg!(alt::Vehicle::get_by_id(id));
    //                         dbg!(vehicle.borrow().id());
    //                         Ok(())
    //                     },
    //                     300,
    //                 );

    //                 Ok(())
    //             },
    //             300,
    //         );
    //         Ok(())
    //     },
    //     300,
    // );

    // alt::events::on("test", |args| {
    //     dbg!(args.get(0));
    //     dbg!(args.get(1));
    //     Ok(())
    // });

    // alt::events::emit!("test", 123i64, alt::ColShape::new_circle(0, 10.0)).unwrap();

    alt::events::on_client("test", |player, args| {
        let p = player;
        let player = p.borrow();
        alt::events::emit_client!("test", p.clone(), "emit single")?;
        alt::events::emit_some_clients!("test", vec![p.clone(), p.clone()], "emit some")?;
        alt::events::emit_all_clients!("test", "emit all")?;

        player.spawn(alt::hash("player_one"), 500);

        let p = p.clone();
        alt::set_timeout(
            move || {
                p.borrow().spawn("player_two", 500);
                Ok(())
            },
            2000,
        );
        Ok(())
    });
}
