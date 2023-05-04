pub use altv::prelude::*;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    let vehicle = altv::Vehicle::new("sultan2", 0, 0)?;
    dbg!(vehicle.stream_synced_meta_keys()?);

    Ok(())
}
