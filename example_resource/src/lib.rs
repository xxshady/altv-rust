pub use altv::prelude::*;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::Vehicle::new("sultan2", 0, 0)?;
    dbg!(altv::Vehicle::all());

    altv::ColShape::new_circle(0, 10.0);
    dbg!(altv::ColShape::all());

    altv::Checkpoint::new(0, 0, 10.0, 10.0, (255, 255, 255), 10);
    dbg!(altv::Checkpoint::all());

    Ok(())
}
