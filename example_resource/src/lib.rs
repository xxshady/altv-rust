pub use altv::prelude::*;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    let vehicle = altv::Vehicle::new("sultan", 0, 0)?;
    dbg!(vehicle.set_mod(0, 0));
    vehicle.set_mod_kit(1)?;
    dbg!(vehicle.set_mod(0, 0));

    dbg!(vehicle.set_mod(255, 255));

    Ok(())
}
