use std::time::Instant;

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    altv::set_interval(
        || {
            let player = altv::__imports::native_player_ped_id().ret;
            let coords = altv::__imports::native_get_entity_coords(player, true).ret;
            
            let res = altv::__imports::native_draw_marker(
                0, coords.x, coords.y, coords.z, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 255, 0, 0, 255,
                false, true, 0, false,
                None,
                None,
                false,
            );

            let veh = altv::__imports::native_get_vehicle_ped_is_in(player, false).ret;
            if veh != 0 {
                let model = altv::__imports::native_get_entity_model(veh).ret;
                let name = altv::__imports::native_get_display_name_from_vehicle_model(model);
                altv::dbg!(name.ret);
            }
            
            // altv::dbg!(res.success, res.ret);
        },
        0,
    );
}
