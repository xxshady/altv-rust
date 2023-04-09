use std::{cell::RefCell, rc::Rc};

pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    // TODO: fix backtrace panic
    std::env::set_var("RUST_BACKTRACE", "full");

    // alt::set_timeout(
    //     move || {
    //         let vehicle = alt::Vehicle::new("sultan2", 3, 0).unwrap();
    //         let id = vehicle.borrow().id().unwrap();
    //         alt::log!("~gl~id: {id}");
    //         let valid = vehicle.borrow().valid();
    //         alt::log!("~gl~valid: {valid}");
    //         alt::log!("~gl~created entity {vehicle:?}");

    //         alt::log!("pos {:?}", vehicle.pos()?);

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

    let maybe_player: RefCell<Option<alt::PlayerContainer>> = RefCell::new(None);

    let maybe_p = maybe_player.clone();
    alt::events::on_client("test", move |player, _args| {
        maybe_p.borrow_mut().replace(player.clone());
        alt::events::emit_client!("test", player.clone(), "emit single")?;
        alt::events::emit_some_clients!("test", vec![player.clone(), player.clone()], "emit some")?;
        alt::events::emit_all_clients!("test", "emit all")?;

        player.spawn("player_two", (0, 0, 72))?;

        let p = player.clone();
        alt::set_timeout(
            move || {
                p.set_dimension(1)?;
                Ok(())
            },
            1500,
        );

        let p = player.clone();
        alt::set_timeout(
            move || {
                p.set_dimension(0)?;
                Ok(())
            },
            2500,
        );

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
            c.col_shape.col_shape_type()?,
            c.vehicle.id().unwrap()
        );
        Ok(())
    });

    alt::events::on_vehicle_leave_col_shape(|c| {
        alt::log!(
            "on_vehicle_leave_col_shape: {:?} {}",
            c.col_shape.col_shape_type()?,
            c.vehicle.id().unwrap()
        );

        Ok(())
    });

    alt::events::on_player_enter_col_shape(|c| {
        alt::log!(
            "on_player_enter_col_shape: {:?} {} {:?}",
            c.col_shape.col_shape_type()?,
            c.player.name()?,
            c.player.pos()?
        );

        Ok(())
    });

    alt::events::on_player_leave_col_shape(|c| {
        alt::log!(
            "on_player_leave_col_shape: {:?} {} {:?}",
            c.col_shape.col_shape_type()?,
            c.player.name()?,
            c.player.pos()?
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
    // poly.set_players_only(true).unwrap();

    alt::ColShape::new_cylinder((9., 5., 70.), 1., 5.);
    alt::ColShape::new_cylinder((9, 5, 70), 1., 5.);

    let veh = alt::Vehicle::new("sultan2", alt::Vector3::new(0., 2., 72.), 0).unwrap();
    let _veh = alt::Vehicle::new("towtruck", alt::Vector3::new(0., 2., 72.), 0).unwrap();

    alt::events::on_console_command(move |c| {
        let "isin" = c.name.as_str() else {
            alt::log!("unknown command: {}", c.name);
            return Ok(());
        };

        alt::log!(
            "player is in: {} veh is in: {}",
            poly.is_entity_in(alt::AnyEntity::Player(
                maybe_player.borrow().as_ref().unwrap().clone()
            ))?,
            poly.is_entity_in(veh.clone())?
        );

        Ok(())
    });

    alt::events::on_weapon_damage(|c| {
        let target = match &c.target {
            alt::AnyEntity::Player(p) => p.name()?,
            _ => unreachable!(),
        };
        let source = c.source.name()?;

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
        let player = c.player.clone();

        let killer = match &c.killer {
            None => "None".to_string(),
            Some(alt::AnyEntity::Player(p)) => format!("player: {}", p.name()?),
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

    macro_rules! ev {
        ($event: ident) => {
            alt::events::$event(|c| {
                alt::log!("{}", stringify!($event));
                dbg!(c);
                Ok(())
            });
        };
    }

    macro_rules! ev_cancel {
        ($event: ident) => {
            alt::events::$event(|c| {
                alt::log!("{}", stringify!($event));
                dbg!(c);
                c.cancel()?;
                Ok(())
            });
        };
    }

    ev_cancel!(on_explosion);
    ev_cancel!(on_start_fire);
    ev_cancel!(on_start_projectile);
    ev!(on_player_weapon_change);
    ev!(on_player_spawn);
    ev!(on_player_damage);
    ev!(on_player_dimension_change);
    ev!(on_player_interior_change);
    ev!(on_player_request_control);
    ev!(on_player_enter_vehicle);

    alt::events::on_player_disconnect(|c| {
        alt::log!("player ~rl~disconnect~w~: {}", c.player.name()?);

        let p = c.player.clone();
        alt::set_interval(
            move || {
                alt::log!("player name: {}", p.name()?);
                Ok(())
            },
            3000,
        );

        Ok(())
    });
}
