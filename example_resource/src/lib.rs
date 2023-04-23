pub use altv::prelude::*;

#[altv::main(crate_name = "altv")]
fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::events::on("test", |args| {
        dbg!(args);
        Ok(())
    });

    altv::events::emit!(
        "test",
        123,
        1.5,
        "test".to_string(),
        altv::mvalue::list![1, 2, 3].unwrap()
    )
    .unwrap();
}
