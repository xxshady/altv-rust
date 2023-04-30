pub use altv::prelude::*;

#[altv::main]
fn main() {
    // std::env::set_var("RUST_BACKTRACE", "full");

    altv::events::on("test", |c| {
        dbg!(c);
    });

    altv::events::emit!("test", Some(1), 2, 3, None as Option<()>).unwrap();
}
