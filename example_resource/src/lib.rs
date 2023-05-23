#![allow(unused_must_use)]

use altv::prelude::*;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::events::on_stream_synced_meta_change(|c| {
        dbg!(c);

        let new_value: Option<i32> = c.new_value.deserialize()?;
        dbg!(new_value);

        Ok(())
    });

    altv::events::on("test", |c| {
        let Some(v) = c.args.get(0) else {
            return Ok(());
        };

        let value: Option<i32> = v.deserialize()?;
        dbg!(value);

        Ok(())
    });

    altv::events::emit("test", &[&Some(123)]);
    altv::events::emit("test", &[&(None as Option<i32>)]);

    let vehicle = altv::Vehicle::new("sultan2", 0, 0)?;

    let already_set_entry = vehicle.stream_synced_meta_entry("already_set")?;

    already_set_entry.set(&228)?;

    // Returns `228` because entry already contained it
    let value = already_set_entry.get_or_set(1337)?;
    assert_eq!(value, 228);

    let empty_entry = vehicle.stream_synced_meta_entry("empty")?;

    // Returns `1337` because entry was empty
    let value = empty_entry.get_or_set(1337)?;
    assert_eq!(value, 1337);

    // Returns `Some(1337)`
    let value: Option<i32> = empty_entry.get()?;
    assert_eq!(value, Some(1337));

    Ok(())
}
