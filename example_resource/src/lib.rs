use std::{cell::RefCell, rc::Rc};

pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    // TODO: fix backtrace panic
    std::env::set_var("RUST_BACKTRACE", "0");

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
    //     alt::log!("args: {:?}", args);
    //     Ok(())
    // });

    // alt::events::emit!(
    //     "test",
    //     123i64,
    //     alt::ColShape::new_circle(0, 10.0),
    //     alt::mvalue::dict!{ "123" => true }.unwrap(),
    //     alt::mvalue::list![false, true].unwrap()
    // )
    // .unwrap();

    let maybe_player: Rc<RefCell<Option<alt::PlayerContainer>>> = Rc::new(RefCell::new(None));

    let maybe_p = maybe_player.clone();
    alt::events::on_client("test", move |player, _args| {
        maybe_p.borrow_mut().replace(player.clone());
        let p = player;
        let player = p.borrow();
        alt::events::emit_client!("test", p.clone(), "emit single")?;
        alt::events::emit_some_clients!("test", vec![p.clone(), p.clone()], "emit some")?;
        alt::events::emit_all_clients!("test", "emit all")?;

        player.spawn("player_two", (0, 0, 72))?;
        Ok(())
    });

    // let group = alt::VirtualEntityGroup::new(3).unwrap();
    // let entity = alt::VirtualEntity::new(group.clone(), 0, 1000).unwrap();
    // let entity_group_id = entity.borrow().group().unwrap().borrow().id().unwrap();
    // dbg!(entity_group_id, group.borrow().id().unwrap());

    // let _col_shape = alt::ColShape::new_circle(0, 3.0);
    alt::events::on_vehicle_enter_col_shape(|c| {
        alt::log!(
            "on_vehicle_enter_col_shape: {:?} {}",
            c.col_shape.try_borrow()?.col_shape_type()?,
            c.vehicle.borrow().id().unwrap()
        );
        Ok(())
    });

    alt::events::on_vehicle_leave_col_shape(|c| {
        alt::log!(
            "on_vehicle_leave_col_shape: {:?} {}",
            c.col_shape.try_borrow()?.col_shape_type()?,
            c.vehicle.borrow().id().unwrap()
        );

        Ok(())
    });

    alt::events::on_player_enter_col_shape(|c| {
        let p = c.player.borrow();
        alt::log!(
            "on_player_enter_col_shape: {:?} {} {:?}",
            c.col_shape.try_borrow()?.col_shape_type()?,
            p.name()?,
            p.pos()?
        );

        Ok(())
    });

    alt::events::on_player_leave_col_shape(|c| {
        let p = c.player.borrow();
        alt::log!(
            "on_player_leave_col_shape: {:?} {} {:?}",
            c.col_shape.try_borrow()?.col_shape_type()?,
            p.name()?,
            p.pos()?
        );

        Ok(())
    });

    // let _vehicle = alt::Vehicle::new("sultan2", alt::Vector3::new(0., 2., 72.), 0).unwrap();

    let poly = alt::ColShape::new_polygon(70., 80., vec![(0., 0.), (0., 3.), (3., 0.)]);
    alt::ColShape::new_cuboid(
        alt::Vector3::new(0., 4., 70.),
        alt::Vector3::new(5., 9., 80.),
    );

    alt::ColShape::new_rectangle(alt::Vector2::new(9., 9.), alt::Vector2::new(15., 15.));
    // poly.borrow().set_players_only(true).unwrap();

    alt::ColShape::new_cylinder((9., 5., 70.), 1., 5.);
    alt::ColShape::new_cylinder((9, 5, 70), 1., 5.);

    let veh = alt::Vehicle::new("sultan2", alt::Vector3::new(0., 2., 72.), 0).unwrap();

    alt::events::on_console_command(move |c| {
        let "isin" = c.name.as_str() else {
            alt::log!("unknown command: {}", c.name);
            return Ok(());
        };

        alt::log!(
            "player is in: {} veh is in: {}",
            poly.try_borrow()?.is_entity_in(alt::AnyEntity::Player(
                maybe_player.borrow().as_ref().unwrap().clone()
            ))?,
            poly.try_borrow()?.is_entity_in(veh.clone())?
        );

        Ok(())
    });

    alt::events::on_weapon_damage(|c| {
        let target = match &c.target {
            alt::AnyEntity::Player(p) => p.try_borrow()?.name()?,
            _ => unreachable!(),
        };
        let source = c.source.try_borrow()?.name()?;

        dbg!(
            "weapon damage",
            source,
            target,
            c.weapon_hash,
            c.damage,
            c.body_part,
            &c.shot_offset,
        );

        c.set_damage(10)?;

        Ok(())
    });

    alt::events::on_player_death(|c| {
        let player = c.player.try_borrow()?;

        let killer = match &c.killer {
            None => "None".to_string(),
            Some(alt::AnyEntity::Player(p)) => format!("player: {}", p.try_borrow()?.name()?),
            Some(alt::AnyEntity::Vehicle(_)) => "vehicle".to_string(),
        };

        alt::log!(
            "player death: {:?} killer: {} weapon: {}",
            player.name()?,
            killer,
            c.weapon_hash
        );

        alt::log!("respawning in 2.5 seconds...");
        let player = c.player.clone();
        alt::set_timeout(
            move || {
                let player = player.try_borrow()?;
                player.spawn(player.model()?, (0, 0, 72))?;
                alt::log!("~gl~respawned");
                Ok(())
            },
            2500,
        );

        Ok(())
    });

    alt::events::on_player_connect_denied(|c| {
        alt::log!("on_player_connect_denied");
        dbg!(c);
        Ok(())
    });
}
