pub use altv::prelude::*;

#[altv::main(crate_name = "altv")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::set_timeout(
        || {
            dbg!(altv::Resource::get_by_name("test")?);
            Ok(())
        },
        500,
    );

    altv::log!("rust resource started");
}
