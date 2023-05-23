use altv::prelude::*;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::events::on("test", |c| {
        let (value, value2, veh, rgba): (Option<i32>, bool, altv::VehicleContainer, altv::Vector2) =
            c.args.deserialize()?;

        dbg!(value, value2, &veh, rgba);
        altv::log!("veh id: {}", veh.id()?);

        Ok(())
    });

    let veh = altv::Vehicle::new("sultan2", 0, 0)?;
    // veh.destroy()?;
    altv::events::emit("test", &[&123, &true, &veh, &altv::RGBA::new(255, 1, 2, 3)])?;
    altv::events::emit(
        "test",
        &[
            &(None as Option<i32>),
            &true,
            &veh,
            &altv::Vector2::new(255, f32::MAX),
        ],
    )?;

    // let entry = altv::meta::entry("test");

    Ok(())
}
