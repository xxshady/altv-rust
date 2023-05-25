use altv::{
    anyhow::{anyhow, bail},
    prelude::*,
    serde::{Deserialize, Serialize},
};
#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "0");

    let rgba: altv::Rgba = 10.into();
    dbg!(rgba);
}
