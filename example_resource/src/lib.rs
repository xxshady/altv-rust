pub use altv::prelude::*;

#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    // if true {
    //     altv::anyhow::bail!("test");
    // }
    // Ok(())
}
