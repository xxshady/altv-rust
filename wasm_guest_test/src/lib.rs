use std::time::Instant;

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    altv::set_interval(
        || {
            altv::__imports::request_streamed_texture_dict(&"mpleaderboard".to_string());
            altv::natives::draw_marker(
                0,
                0.0,
                0.0,
                71.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0,
                1.0,
                1.0,
                1.0,
                255,
                0,
                0,
                180,
                false,
                true,
                0,
                false,
                &"mpleaderboard".to_string(),
                &"leaderboard_audio_3".to_string(),
                false,
            );
        },
        0,
    );

    // altv::set_interval(
    //     move || {
    //         let now = Instant::now();
    //         bench();
    //         altv::dbg!(now.elapsed());
    //     },
    //     1500,
    // );
}

fn bench() {
    use altv::__imports::*;

    let sound_name = "win".to_string();
    let sound_ref = "dlc_vw_casino_lucky_wheel_sounds".to_string();

    for _ in 1..=100 {
        for _ in 1..=90 {
            let id = natives_get_sound_id();
            natives_play_sound_from_coord(
                id,
                &sound_name,
                1111.052,
                229.8579,
                -49.133,
                &sound_ref,
                0,
                0,
                0,
            );

            natives_release_sound_id(id);
        }
    }
}
