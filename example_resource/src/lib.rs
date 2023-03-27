pub use alt::prelude::*;
use autocxx::prelude::*;

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

    // alt::events::on_player_connect(|alt::events::sdk_controllers::PlayerConnect { player }| {
    //     {
    //         let mut player = player.try_borrow_mut()?;
    //         let id = player.id()?;

    //         alt::log!(
    //             "example resource on_player_connect name: {} id: {}",
    //             player.name().unwrap(),
    //             id
    //         );
    //     }

    //     // alt::set_interval(
    //     //     move || {
    //     //         let vehicle = alt::Vehicle::new(alt::hash("sultan2"), 0.into(), 0.into()).unwrap();
    //     //         alt::events::emit_client!(
    //     //             "test",
    //     //             player.clone(),
    //     //             "test",
    //     //             123u64,
    //     //             alt::events::list![vehicle].unwrap()
    //     //         )
    //     //         .unwrap();

    //     //         alt::events::emit_client!("test", player.clone()).unwrap();

    //     //         alt::events::emit_all_clients!("test", "emit all", player.clone()).unwrap();

    //     //         alt::events::emit_some_clients!(
    //     //             "test",
    //     //             vec![player.clone()],
    //     //             "emit some",
    //     //             player.clone()
    //     //         )
    //     //         .unwrap();

    //     //         Ok(())
    //     //     },
    //     //     1000,
    //     // );

    //     Ok(())
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

    // alt::set_timeout(
    //     || {
    //         alt::log!("~cl~start");

    //         alt::events::on("test".to_string(), |args| {
    //             alt::log!("test event args: {:?}", args);
    //             // args.get_dict_at(4)?;
    //             Ok(())
    //         });

    //         for _ in 0..1 {
    //             let mut str = String::from("");
    //             for _ in 1..100 {
    //                 let a: i32 = rand::random();
    //                 str += a.to_string().as_str();
    //             }

    //             alt::set_timeout(
    //                 move || {
    //                     let vehicle =
    //                         alt::Vehicle::new(alt::hash("sultan2"), 0.into(), 0.into()).unwrap();

    //                     // vehicle.borrow_mut().destroy().unwrap();

    //                     alt::events::emit!("js-destroy-veh", vehicle.clone()).unwrap();

    //                     alt::set_timeout(
    //                         move || {
    //                             alt::events::emit!(
    //                                 "test",
    //                                 69i64,
    //                                 true,
    //                                 "string",
    //                                 // TODO: make this shit more user-friendly if dict! or list! result is passed directly without unwrapping it
    //                                 alt::events::dict! {
    //                                     "kek" => 123i64,
    //                                     // "vehicle" => vehicle.clone(),
    //                                 }
    //                                 .unwrap()
    //                             )
    //                             .unwrap();

    //                             Ok(())
    //                         },
    //                         500,
    //                     );

    //                     Ok(())
    //                 },
    //                 500,
    //             );

    //             // vehicle.borrow_mut().destroy().unwrap();
    //         }

    //         alt::log!("~gl~done");

    //         Ok(())
    //     },
    //     100,
    // );

    // debug log should be printed that event is unhandled
    // alt::events::emit!("lalallaall", alt::events::dict! { "test" => true }.unwrap()).unwrap();

    // alt::events::on_client("test".to_string(), |player_rc, args| {
    //     alt::log!("on client test player: {}", player_rc.try_borrow()?.name()?);
    //     alt::log!("on client test args: {:#?}", args);

    //     let mut player = player_rc.try_borrow_mut()?;

    //     let model = alt::hash("mp_m_freemode_01");

    //     // player.spawn(model, alt::Vector3::new(0f32, 0f32, 72f32))?;

    //     dbg!(player.model()? == model);

    //     unsafe {
    //         alt::ffi::IPlayer::GiveWeapon(
    //             player.ptr_mut().to_player()?,
    //             alt::hash("weapon_pistol"),
    //             100,
    //             true,
    //         )
    //     }

    //     let player_inter = player_rc.clone();
    //     let mut hour = 0;

    //     let veh = alt::Vehicle::new(
    //         alt::hash("sultan3"),
    //         alt::Vector3::new(0f32, 0f32, 72f32),
    //         0.into(),
    //     )
    //     .unwrap();

    //     unsafe {
    //         let ptr = player.ptr_mut().to_player()?;
    //         alt::ffi::IPlayer::SetHeadBlendData(ptr, 1, 2, 3, 4, 5, 6, 7.0, 8.0, 9.0);

    //         let data = alt::ffi::IPlayer::GetHeadBlendData(ptr).within_unique_ptr();

    //         let out_shape_first_id = Box::into_raw(Box::new(0u32));
    //         let out_shape_second_id = Box::into_raw(Box::new(0u32));
    //         let out_shape_third_id = Box::into_raw(Box::new(0u32));
    //         let out_skin_first_id = Box::into_raw(Box::new(0u32));
    //         let out_skin_second_id = Box::into_raw(Box::new(0u32));
    //         let out_skin_third_id = Box::into_raw(Box::new(0u32));
    //         let out_shape_mix = Box::into_raw(Box::new(0f32));
    //         let out_skin_mix = Box::into_raw(Box::new(0f32));
    //         let out_third_mix = Box::into_raw(Box::new(0f32));
    //         alt::ffi::read_alt_head_blend_data(
    //             data.as_ref().unwrap(),
    //             out_shape_first_id,
    //             out_shape_second_id,
    //             out_shape_third_id,
    //             out_skin_first_id,
    //             out_skin_second_id,
    //             out_skin_third_id,
    //             out_shape_mix,
    //             out_skin_mix,
    //             out_third_mix,
    //         );

    //         dbg!(
    //             *out_shape_first_id,
    //             *out_shape_second_id,
    //             *out_shape_third_id,
    //             *out_skin_first_id,
    //             *out_skin_second_id,
    //             *out_skin_third_id,
    //             *out_shape_mix,
    //             *out_skin_mix,
    //             *out_third_mix
    //         );

    //         alt::ffi::IPlayer::SetHeadOverlay(ptr, 8, 3, 0.3);
    //         alt::ffi::IPlayer::SetHeadOverlayColor(ptr, 8, 2, 2, 1);

    //         let overlay = alt::ffi::IPlayer::GetHeadOverlay(ptr, 8).within_unique_ptr();

    //         let out_index = Box::into_raw(Box::new(0));
    //         let out_opacity = Box::into_raw(Box::new(0.0));
    //         let out_color_type = Box::into_raw(Box::new(0));
    //         let out_color_index = Box::into_raw(Box::new(0));
    //         let out_second_color_index = Box::into_raw(Box::new(0));

    //         alt::ffi::read_alt_head_overlay(
    //             overlay.as_ref().unwrap(),
    //             out_index,
    //             out_opacity,
    //             out_color_type,
    //             out_color_index,
    //             out_second_color_index,
    //         );

    //         println!("head overlay read:");

    //         dbg!(
    //             *out_index,
    //             *out_opacity,
    //             *out_color_type,
    //             *out_color_index,
    //             *out_second_color_index
    //         );

    //         alt::ffi::IPlayer::SetLocalMetaData(
    //             ptr,
    //             "test",
    //             alt::ffi::create_mvalue_bool(true).within_unique_ptr(),
    //         );
    //         alt::ffi::IPlayer::SetLocalMetaData(
    //             ptr,
    //             "test2",
    //             alt::ffi::create_mvalue_bool(false).within_unique_ptr(),
    //         );

    //         dbg!(alt::ffi::IPlayer::GetLocalMetaDataKeys(ptr));
    //     }

    //     // alt::set_interval(
    //     //     move || {
    //     //         let mut player = player_inter.try_borrow_mut()?;
    //     //         hour = if hour == 23 { 0 } else { hour + 1 };
    //     //         unsafe {
    //     //             alt::ffi::IPlayer::GiveWeapon(
    //     //                 player.ptr_mut().to_player()?,
    //     //                 alt::hash("weapon_smg"),
    //     //                 100,
    //     //                 true,
    //     //             );
    //     //             alt::ffi::IPlayer::SetDateTime(
    //     //                 player.ptr_mut().to_player()?,
    //     //                 0.into(),
    //     //                 0.into(),
    //     //                 0.into(),
    //     //                 hour.into(),
    //     //                 0.into(),
    //     //                 0.into(),
    //     //             );

    //     //             alt::ffi::IPlayer::SetLocalMetaData(
    //     //                 player.ptr_mut().to_player()?,
    //     //                 "test",
    //     //                 alt::ffi::create_mvalue_bool(true).within_unique_ptr(),
    //     //             );

    //     //             alt::ffi::IPlayer::SetProps(player.ptr_mut().to_player()?, 6, 0, 1);

    //     //             let prop = alt::ffi::IPlayer::GetProps(player.ptr_mut().to_player()?, 6)
    //     //                 .within_unique_ptr();

    //     //             let out_drawable = Box::into_raw(Box::new(0u16));
    //     //             let out_texture = Box::into_raw(Box::new(0u8));

    //     //             alt::ffi::read_alt_prop(prop.as_ref().unwrap(), out_drawable, out_texture);
    //     //             alt::log_error!(
    //     //                 "prop drawable: {:?} texture: {:?}",
    //     //                 *out_drawable,
    //     //                 *out_texture
    //     //             );

    //     //             let clothes_result =
    //     //                 alt::ffi::IPlayer::SetClothes(player.ptr_mut().to_player()?, 11, 1, 1, 0);
    //     //             dbg!(clothes_result);

    //     //             let cloth = alt::ffi::IPlayer::GetClothes(player.ptr_mut().to_player()?, 11)
    //     //                 .within_unique_ptr();

    //     //             let out_drawable = Box::into_raw(Box::new(0u16));
    //     //             let out_texture = Box::into_raw(Box::new(0u8));
    //     //             let out_palette = Box::into_raw(Box::new(0u8));

    //     //             alt::ffi::read_alt_cloth(
    //     //                 cloth.as_ref().unwrap(),
    //     //                 out_drawable,
    //     //                 out_texture,
    //     //                 out_palette,
    //     //             );
    //     //             alt::log_error!(
    //     //                 "cloth drawable: {:?} texture: {:?} palette: {:?}",
    //     //                 *out_drawable,
    //     //                 *out_texture,
    //     //                 *out_palette
    //     //             );

    //     //             let dlc_clothes_result = alt::ffi::IPlayer::SetDlcClothes(
    //     //                 player.ptr_mut().to_player()?,
    //     //                 11,
    //     //                 3,
    //     //                 2,
    //     //                 0,
    //     //                 0,
    //     //             );
    //     //             dbg!(dlc_clothes_result);

    //     //             let dlc_cloth =
    //     //                 alt::ffi::IPlayer::GetDlcClothes(player.ptr_mut().to_player()?, 11)
    //     //                     .within_unique_ptr();

    //     //             let out_drawable = Box::into_raw(Box::new(0u16));
    //     //             let out_texture = Box::into_raw(Box::new(0u8));
    //     //             let out_palette = Box::into_raw(Box::new(0u8));
    //     //             let out_dlc = Box::into_raw(Box::new(0u32));

    //     //             alt::ffi::read_alt_dlc_cloth(
    //     //                 dlc_cloth.as_ref().unwrap(),
    //     //                 out_drawable,
    //     //                 out_texture,
    //     //                 out_palette,
    //     //                 out_dlc,
    //     //             );
    //     //             alt::log_error!(
    //     //                 "dlc cloth drawable: {:?} texture: {:?} palette: {:?} dlc: {:?}",
    //     //                 *out_drawable,
    //     //                 *out_texture,
    //     //                 *out_palette,
    //     //                 *out_dlc
    //     //             );

    //     //             let aim_pos = alt::ffi::IPlayer::GetAimPos(player.ptr_mut().to_player()?)
    //     //                 .within_unique_ptr();

    //     //             let out_x = Box::into_raw(Box::new(0f32));
    //     //             let out_y = Box::into_raw(Box::new(0f32));
    //     //             let out_z = Box::into_raw(Box::new(0f32));
    //     //             alt::ffi::read_vector3(aim_pos.as_ref().unwrap(), out_x, out_y, out_z);

    //     //             alt::log_error!("aim pos: {:?} {:?} {:?}", *out_x, *out_y, *out_z,);

    //     //             let left = Box::into_raw(Box::new(false));
    //     //             let right = Box::into_raw(Box::new(false));
    //     //             let front = Box::into_raw(Box::new(false));
    //     //             let back = Box::into_raw(Box::new(false));
    //     //             alt::ffi::IVehicle::GetNeonActive(
    //     //                 veh.borrow_mut().ptr_mut().to_vehicle()?,
    //     //                 left,
    //     //                 right,
    //     //                 front,
    //     //                 back,
    //     //             );
    //     //             alt::log!(
    //     //                 "neon left: {:?}, right: {:?}, front: {:?}, back: {:?}",
    //     //                 *left,
    //     //                 *right,
    //     //                 *front,
    //     //                 *back
    //     //             );

    //     //             alt::ffi::IVehicle::SetNeonColor(
    //     //                 veh.borrow_mut().ptr_mut().to_vehicle()?,
    //     //                 rand::random(),
    //     //                 rand::random(),
    //     //                 rand::random(),
    //     //                 255,
    //     //             );

    //     //             alt::ffi::IVehicle::SetNeonActive(
    //     //                 veh.borrow_mut().ptr_mut().to_vehicle()?,
    //     //                 !*left,
    //     //                 !*right,
    //     //                 !*front,
    //     //                 !*back,
    //     //             );

    //     //             alt::ffi::IVehicle::SetPrimaryColorRGB(
    //     //                 veh.borrow_mut().ptr_mut().to_vehicle()?,
    //     //                 255,
    //     //                 0,
    //     //                 0,
    //     //                 255,
    //     //             );

    //     //             alt::ffi::IVehicle::SetSecondaryColorRGB(
    //     //                 veh.borrow_mut().ptr_mut().to_vehicle()?,
    //     //                 255,
    //     //                 0,
    //     //                 255,
    //     //                 255,
    //     //             );

    //     //             let rgba = alt::ffi::IVehicle::GetSecondaryColorRGB(
    //     //                 veh.borrow_mut().ptr_mut().to_vehicle()?,
    //     //             );

    //     //             let out_r = Box::into_raw(Box::from(0u8));
    //     //             let out_g = Box::into_raw(Box::from(0u8));
    //     //             let out_b = Box::into_raw(Box::from(0u8));
    //     //             let out_a = Box::into_raw(Box::from(0u8));
    //     //             alt::ffi::read_rgba(
    //     //                 rgba.within_unique_ptr().as_ref().unwrap(),
    //     //                 out_r,
    //     //                 out_g,
    //     //                 out_b,
    //     //                 out_a,
    //     //             );

    //     //             alt::log!(
    //     //                 "GetSecondaryColorRGB rgba {:?}, {:?}, {:?}, {:?}",
    //     //                 *out_r,
    //     //                 *out_g,
    //     //                 *out_b,
    //     //                 *out_a
    //     //             );

    //     //             // alt::ffi::IPlayer::Spawn(
    //     //             //     player.ptr_mut().to_player()?,
    //     //             //     hour as f32,
    //     //             //     0 as f32,
    //     //             //     72 as f32,
    //     //             //     0,
    //     //             // );

    //     //             // let driver =
    //     //             //     alt::ffi::IVehicle_GetDriver(veh.borrow_mut().ptr_mut().to_vehicle()?);
    //     //             // alt::log!(
    //     //             //     "vehicle driver: {:?} {:?}",
    //     //             //     driver,
    //     //             //     driver == player.ptr_mut().to_player()?
    //     //             // );

    //     //             // let velocity =
    //     //             //     alt::ffi::IVehicle_GetVelocity(veh.borrow_mut().ptr_mut().to_vehicle()?)
    //     //             //         .within_unique_ptr();
    //     //             // alt::log_error!(
    //     //             //     "veh velocity: {:?} {:?} {:?}",
    //     //             //     alt::ffi::read_vector3_x(velocity.as_ref().unwrap()),
    //     //             //     alt::ffi::read_vector3_y(velocity.as_ref().unwrap()),
    //     //             //     alt::ffi::read_vector3_z(velocity.as_ref().unwrap()),
    //     //             // );
    //     //             // alt::events::emit!(
    //     //             //     "js-veh-velocity",
    //     //             //     alt::events::dict!("kek" => "kek").unwrap()
    //     //             // )
    //     //             // .unwrap();

    //     //             alt::ffi::IPlayer::SetHeadOverlay(ptr, overlayID, index, opacity)
    //     //         }
    //     //         Ok(())
    //     //     },
    //     //     700,
    //     // );

    //     Ok(())
    // });

    // alt::events::on_console_command(
    //     |alt::events::ConsoleCommandController { name, args: _args }| {
    //         if name != "weapon" {
    //             return;
    //         }
    //         alt::log!("weapon cmd");

    //         let player = alt::Player::get_by_id(0).unwrap();
    //         let mut player = player.borrow_mut();

    //         unsafe {
    //             let ptr = player.ptr_mut().to_player().unwrap();

    //             let smg = alt::hash("weapon_smg");
    //             alt::ffi::IPlayer::GiveWeapon(ptr, smg, 100, true);
    //             alt::ffi::IPlayer::SetWeaponTintIndex(ptr, smg, 3);

    //             let pistol = alt::hash("weapon_pistol");
    //             alt::ffi::IPlayer::GiveWeapon(ptr, pistol, 100, true);
    //             alt::ffi::IPlayer::AddWeaponComponent(ptr, pistol, 0xD7391086);
    //             alt::ffi::IPlayer::AddWeaponComponent(ptr, pistol, 0xED265A1C);
    //             // alt::ffi::IPlayer::RemoveWeaponComponent(ptr, pistol, 0xED265A1C);

    //             alt::set_timeout(
    //                 move || {
    //                     let current_weapon = alt::ffi::IPlayer::GetCurrentWeapon(ptr);
    //                     let components = alt::ffi::IPlayer::GetCurrentWeaponComponents(ptr);
    //                     dbg!(current_weapon, current_weapon == pistol);
    //                     dbg!(0xD7391086u32);
    //                     dbg!(0xED265A1Cu32);
    //                     dbg!(components);

    //                     let weapons = alt::ffi::IPlayer::GetWeapons(ptr);
    //                     for w in weapons.iter() {
    //                         let out_hash = Box::into_raw(Box::new(0));
    //                         let out_tint_index = Box::into_raw(Box::new(0));
    //                         alt::ffi::read_weapon(w, out_hash, out_tint_index);
    //                         dbg!(
    //                             *out_hash,
    //                             *out_tint_index,
    //                             *out_hash == alt::hash("weapon_pistol"),
    //                             *out_hash == alt::hash("weapon_smg"),
    //                         );
    //                         let comps = alt::ffi::read_weapon_components(w);
    //                         dbg!(comps);
    //                     }

    //                     Ok(())
    //                 },
    //                 1000,
    //             );
    //         }
    //     },
    // );

    // world object stuff

    // let veh = alt::Vehicle::new(alt::hash("sultan3"), 0.into(), 0.into()).unwrap();
    // let veh = veh.borrow();

    // dbg!(veh.pos());
    // dbg!(veh.set_pos(alt::Vector3::new(1.0, 2.0, 3.0)));
    // dbg!(veh.pos());

    // dbg!(veh.dimension());
    // dbg!(veh.set_dimension(1));
    // dbg!(veh.dimension());

    // vector2 & vector3 mvalue

    // alt::events::on("test".to_string(), |args| {
    //     dbg!(args);
    //     Ok(())
    // });

    // alt::events::on("test".to_string(), |args| {
    //     alt::log!("second local event test handler");
    //     dbg!(args);
    //     Ok(())
    // });

    // dbg!(alt::events::emit!("test", alt::Vector3::new(0.0, 1.0, 2.0)));

    // dbg!(alt::events::emit!("test", alt::Vector2::new(0.0, 1.0)));

    // dbg!(alt::events::emit!(
    //     "testdddddddd",
    //     alt::Vector2::new(0.0, 1.0)
    // ));

    // let mut col_shape = alt::ColShape::new_circle(0.into(), 10.0);

    // let veh = alt::Vehicle::new(alt::hash("sultan"), 0.into(), 0.into()).unwrap();
    // dbg!(veh);

    use alt::events;

    // events::on_vehicle_enter_col_shape(|c| {
    //     alt::log!("~gl~vehicle enter colshape");
    //     Ok(())
    // });

    // events::on_vehicle_leave_col_shape(|c| {
    //     alt::log!("~rl~vehicle leave colshape");
    //     Ok(())
    // });

    // let col = col_shape.clone();
    // alt::set_timeout(
    //     move || {
    //         dbg!();
    //         col.try_borrow()?.set_pos(11.0.into())?;
    //         Ok(())
    //     },
    //     500,
    // );

    // let col = col_shape.clone();
    // alt::set_timeout(
    //     move || {
    //         dbg!();
    //         col.try_borrow()?.set_pos(0.0.into())?;

    //         Ok(())
    //     },
    //     1000,
    // );

    // let col = col_shape.clone();
    // alt::set_timeout(
    //     move || {
    //         dbg!();
    //         col.try_borrow_mut()?.destroy()?;
    //         Ok(())
    //     },
    //     1500,
    // );

    alt::events::on_console_command(
        |alt::events::sdk_controllers::ConsoleCommandEvent { name, args }| {
            alt::log!("on_console_command name: {name:?} args: {args:?}");

            if name.as_str() == "get-veh" {
                let id = args.get(0).ok_or(alt::anyhow::anyhow!("expected id"))?;
                let id: alt::EntityId = id.parse()?;
                let veh = alt::Vehicle::get_by_id(id).ok_or(alt::anyhow::anyhow!("no veh"))?;
                alt::log!("veh: {veh:?}");
            }

            Ok(())
        },
    );

    // events::on_client("test".to_string(), |player, args| {
    //     dbg!(player, args);
    //     Ok(())
    // });

    // events::on_server_started(|controller| {
    //     alt::log_warn!("example resource on_server_started controller: {controller:?}");
    //     Ok(())
    // });

    events::on_player_connect(|events::sdk_controllers::PlayerConnect { player }| {
        let player = player.borrow();
        alt::log!(
            "on_player_connect player: {} [{}]",
            player.name()?,
            player.id()?,
        );
        let model = alt::hash("mp_m_freemode_01");
        player.spawn(model, alt::Vector3::new(0., 0., 72.))?;
        dbg!(player.model()? == model);
        Ok(())
    });

    // events::on_player_disconnect(
    //     |events::sdk_controllers::PlayerDisconnect { player, reason }| {
    //         let player = player.borrow();
    //         alt::log!(
    //             "on_player_disconnect player: {} [{}], reason: {}",
    //             player.name()?,
    //             player.id()?,
    //             reason
    //         );
    //         Ok(())
    //     },
    // );

    let group = alt::VirtualEntityGroup::new(10);
    let entity = alt::VirtualEntity::new(group.clone(), alt::Vector3::new(0., 0., 72.), 5).unwrap();

    dbg!(entity.borrow().group().unwrap().borrow().id());
    dbg!(group.borrow().id());
}
