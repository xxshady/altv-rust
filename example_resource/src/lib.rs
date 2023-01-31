#[alt::res_main]
#[no_mangle]
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

    let mut i = 0;
    alt::events::on_server_started(move |controller| {
        i += 1;
        alt::log!("example resource ServerStarted controller: {controller:?} i: {i:?}");
    });

    alt::events::on_player_disconnect(|controller| {
        alt::log!("example resource on_player_disconnect controller: {controller:?}");
    });

    // alt::events::on(alt::events::Event::PlayerConnect(|data| {
    //     alt::log!(
    //         "example resource PlayerConnect data player: {:?} reason: {:?}",
    //         data.player,
    //         data.reason
    //     );
    // }));

    // alt::set_timeout(|| println!("its timeout"), 300);

    // fn on_player_connect(player_ptr: usize) {
    //     alt::log_warn!("player connect player_ptr: {:?}", player_ptr);
    // }

    // alt::events::on(alt::events::__test_SDKEvent::ServerStarted(|| {
    //     alt::log!("on_server_started test");
    // }));

    // fn on_server_started2() {
    //     alt::log_warn!("on_server_started 2");
    // }

    // alt::events::on(alt::__resource_api::events::SDKEvent::PlayerConnect(
    //     on_player_connect,
    // ));

    // alt::events::on(alt::__resource_api::events::SDKEvent::ServerStarted(
    //     on_server_started2,
    // ));

    // fn on_player_disconnect(player: usize, reason: String) {
    //     alt::log!("on_player_disconnect {:?} {:?}", player, reason);
    // }

    // alt::events::on(alt::__resource_api::events::SDKEvent::PlayerDisconnect(
    //     on_player_disconnect,
    // ));

    alt::set_timeout(
        || {
            let vehicle = alt::create_vehicle(alt::hash("sultan"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
            let binding = vehicle.unwrap();
            let binding = binding.try_lock().unwrap();
            let vehicle = binding.as_ref().unwrap();

            dbg!(vehicle.id());
        },
        2000,
    );
}
