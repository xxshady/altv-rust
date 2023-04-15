pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    for _ in 0..50_000 {
        alt::Vehicle::new("s80", 0, 0).unwrap();
    }

    alt::set_timeout(
        move || {
            alt::log!("destroying vehicles");
            alt::Vehicle::all().iter().for_each(|v| {
                v.destroy().unwrap();
            });
            alt::log!("destroyed vehicles");
            Ok(())
        },
        1000,
    );

    alt::set_password("test");
}
