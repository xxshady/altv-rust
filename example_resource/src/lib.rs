pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    alt::set_timeout(
        || {
            dbg!(alt::Resource::get_by_name("test")?);
            Ok(())
        },
        500,
    );

    alt::log!("rust resource started");
}
