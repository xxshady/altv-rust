pub(crate) fn test_timers() {
    let mut timer1 = altv::set_timeout(
        || {
            altv::log!("timer1");
        },
        200,
    );

    let mut timer2 = altv::set_timeout(
        || {
            altv::log!("timer2");
        },
        200,
    );

    altv::set_timeout(
        || {
            altv::log!("timer3");
        },
        200,
    );

    altv::set_timeout(
        || {
            altv::log!("timer4");
        },
        200,
    );

    altv::set_timeout(
        move || {
            timer1.destroy().unwrap();
            timer2.destroy().unwrap();
        },
        100,
    );
}
