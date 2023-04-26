use std::collections::HashMap;

pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    altv::events::on("test", |args| {
        dbg!(args);
        Ok(())
    });

    altv::events::on("test", |args| {
        dbg!(args);
        Ok(())
    });

    altv::events::emit!("test", 1, 2, 3).unwrap();
    altv::events::emit!("test3", 1, 2, 3).unwrap();
}
