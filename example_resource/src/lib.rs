pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let vehicle = alt::Vehicle::new("s80", 0, 0).unwrap();
    alt::set_timeout(
        move || {
            alt::stop_server();
            Ok(())
        },
        1000,
    );
}
