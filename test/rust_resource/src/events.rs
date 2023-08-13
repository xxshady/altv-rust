pub(crate) fn test_events() {
    altv::events::on_voice_connection(|c| {
        altv::log!("~gl~voice_connection {c:?}");
    });

    altv::events::on_voice_connecting(|c| {
        altv::log!("~gl~voice_connecting {c:?}");
    });

    altv::events::on_voice_connect(|c| {
        altv::log!("~gl~voice_connect {c:?}");
    });

    altv::events::on_voice_disconnect(|c| {
        altv::log!("~gl~voice_disconnect {c:?}");
    });
}
