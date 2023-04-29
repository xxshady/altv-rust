pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    altv::events::on("test", |c| {
        dbg!(c);
        altv::Vehicle::new("s80", 0, 0).unwrap();
    });

    altv::events::on("test", |c| {
        dbg!(c);
        altv::Vehicle::new("s80", 0, 0)?;
        Ok(())
    });

    altv::events::on_client("test", |c| {
        dbg!(c);
        altv::Vehicle::new("s80", 0, 0).unwrap();
    });

    altv::events::on_client("test", |c| {
        dbg!(c);
        altv::Vehicle::new("s80", 0, 0)?;
        Ok(())
    });

    altv::events::emit!("test", 123).unwrap();

    altv::events::on_player_connect(|c| {
        dbg!(c);
    });

    altv::events::on_player_connect(|c| {
        dbg!(c);
        Ok(())
    });

    altv::events::on_vehicle_enter_col_shape(|c| {
        dbg!(c);
    });

    altv::events::on_vehicle_enter_col_shape(|c| {
        dbg!(c);
        altv::Vehicle::new("s80", 0, 0)?;
        Ok(())
    });

    altv::events::on_player_connect(|c| {
        dbg!(c);
        Ok(())
    });
}
