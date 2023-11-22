use std::time::Instant;

use bincode::Options;

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    for i in 0..10 {
        let buf = altv::MemoryBuffer::new(312).unwrap();
        let res = altv::__imports::native_get_dlc_weapon_data(i, buf.id());
        altv::dbg!(res.ret);

        let buf = buf.read();
        // let data: (i32, i32, i32, i32, i32, i32, i32) = bincode::DefaultOptions::new()
        //     .with_little_endian()
        //     .allow_trailing_bytes()
        //     .deserialize(&buf)
        //     .unwrap();
        // let data: (i32, i32, i32, i32, i32, i32, i32) = bincode::deserialize(&buf).unwrap();
        // altv::log!("data: {data:?}");
        // altv::log!("weapon_hash in data: {:#x}", data.2);

        // let weapon_hash = i32::from_le_bytes(buf[8..12].try_into().unwrap());
        // altv::log!("weapon_hash: {weapon_hash:#x}");

        altv::dbg!(&buf[54..56 + 64]);
    }

    // altv::set_interval(
    //     || {
    //         let player = altv::__imports::native_player_ped_id().ret;
    //         let coords = altv::__imports::native_get_entity_coords(player, true).ret;

    //         let res = altv::__imports::native_draw_marker(
    //             0, coords.x, coords.y, coords.z, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 255, 0, 0, 255,
    //             false, true, 0, false,
    //             None,
    //             None,
    //             false,
    //         );

    //         let veh = altv::__imports::native_get_vehicle_ped_is_in(player, false).ret;
    //         if veh != 0 {
    //             let model = altv::__imports::native_get_entity_model(veh).ret;
    //             let name = altv::__imports::native_get_display_name_from_vehicle_model(model);
    //             altv::dbg!(name.ret);
    //         }

    //         // altv::dbg!(res.success, res.ret);
    //     },
    //     0,
    // );
}
