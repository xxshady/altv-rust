#[alt::res_main]
#[no_mangle]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let mut i = 0;
    alt::set_interval(
        move || {
            i += 1;
            alt::log!("test interval i: {i}");
        },
        1000,
    );

    // alt::set_timeout(|| println!("its timeout"), 300);

    // fn on_player_connect(player_ptr: usize) {
    //     alt::log_warn!("player connect player_ptr: {:?}", player_ptr);
    // }

    // fn on_server_started1() {
    //     alt::log_warn!("on_server_started 1");
    // }

    // fn on_server_started2() {
    //     alt::log_warn!("on_server_started 2");
    // }

    // alt::events::on(alt::__resource_api::events::SDKEvent::PlayerConnect(
    //     on_player_connect,
    // ));

    // alt::events::on(alt::__resource_api::events::SDKEvent::ServerStarted(
    //     on_server_started1,
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
}
