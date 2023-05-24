#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::events::on_connection_queue_add(|c| {
        dbg!(c);
        dbg!(c.info.name());

        let info = c.info.clone();
        altv::set_timeout(
            move || {
                dbg!(info.name());
                info.accept(true)?;
                info.decline("kek")?;
                Ok(())
            },
            2000,
        );
    });

    altv::events::on_connection_queue_remove(|c| {
        dbg!(c);
        dbg!(c.info.name());

        let info = c.info.clone();
        altv::set_timeout(
            move || {
                dbg!(info.name());
                info.accept(true)?;
                Ok(())
            },
            2000,
        );
    });

    let veh = altv::Vehicle::new("sultan2", 0, 0).unwrap();
    dbg!(veh.id()?);
    let veh = altv::Vehicle::new("sultan2", 0, 0).unwrap();
    dbg!(veh.id()?);
    let veh = altv::Vehicle::new("sultan2", 0, 0).unwrap();
    dbg!(veh.id()?);
    let veh = altv::Vehicle::new("sultan2", 0, 0).unwrap();
    dbg!(veh.id()?);
    let veh = altv::Vehicle::new("sultan2", 0, 0).unwrap();
    dbg!(veh.id()?);

    Ok(())
}
