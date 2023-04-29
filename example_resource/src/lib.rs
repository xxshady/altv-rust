pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    let obj = altv::NetworkObject::new_with_params(
        "prop_bench_04",
        (0, 0, 74),
        (1.0, 2.0, 3.0),
        150,
        0,
        2,
    )
    .unwrap();

    obj.place_on_ground_properly().unwrap();

    altv::events::on_player_connect(move |_| {
        let obj = obj.clone();
        altv::set_timeout(
            move || {
                altv::log!("destroying");
                obj.destroy()?;
                Ok(())
            },
            5000,
        );
    })
}
