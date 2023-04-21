pub use altv::prelude::*;

#[altv::main(crate_name = "altv")]
fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    let config = altv::ServerConfig::get();
    dbg!(config);

    let res = altv::Resource::current();
    dbg!(res.config.get_custom_key("type"));
}
