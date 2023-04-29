pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    altv::events::on("test", |c| {
        dbg!(c);
        Ok(())
    });

    altv::events::on_client("test", |c| {
        dbg!(c);
        Ok(())
    });

    altv::events::emit!("test", 123).unwrap();
}
