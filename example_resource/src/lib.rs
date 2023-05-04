pub use altv::prelude::*;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    // vehicle();

    altv::set_interval(
        || {
            dbg!(altv::Vehicle::all());
        },
        1000,
    );

    // altv::set_timeout(
    //     || {
    //         vehicle();
    //         // checkpoint();
    //         dbg!(altv::Checkpoint::all());
    //     },
    //     200,
    // );

    fn checkpoint() {
        let ch = altv::Checkpoint::new(0, 0, 10.0, 10.0, (255, 255, 255), 10);

        dbg!(ch.valid());

        dbg!(altv::Checkpoint::all());

        altv::set_timeout(
            move || {
                dbg!(ch.valid());

                altv::events::emit!("destroy");

                let ch = ch.clone();
                altv::set_timeout(
                    move || {
                        dbg!(ch.valid());
                        dbg!(ch.destroy().expect_err("should be err"));
                        dbg!(altv::Checkpoint::all());

                        // vehicle();
                    },
                    200,
                )
            },
            200,
        );
    }

    fn vehicle() {
        let veh = altv::Vehicle::new("sultan", 0, 0).unwrap();

        dbg!(veh.valid());

        dbg!(altv::Vehicle::all());

        // altv::set_timeout(
        //     move || {
        dbg!(veh.valid());

        // altv::events::emit!("destroy");
        altv::Vehicle::all()
            .iter()
            .for_each(|v| v.destroy().unwrap());

        dbg!(veh.valid());
        dbg!(veh.destroy().expect_err("should be err"));
        dbg!(altv::Vehicle::all());
        //     },
        //     200,
        // );
    }

    Ok(())
}
