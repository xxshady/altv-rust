use altv::prelude::*;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::events::on("test", |c| {
        let (value, value2, veh): (i32, bool, altv::VehicleContainer) = c.args.deserialize()?;

        dbg!(value, value2, &veh);
        altv::log!("veh id: {}", veh.id()?);

        Ok(())
    });

    let veh = altv::Vehicle::new("sultan2", 0, 0)?;
    veh.destroy()?;
    altv::events::emit("test", &[&123, &true, &veh])?;
    altv::events::emit("test", &[&(None as Option<i32>)])?;

    Ok(())
}
