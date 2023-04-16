pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
}
