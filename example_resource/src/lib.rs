#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    use altv::mvalue::{from_mvalue, to_mvalue};

    Ok(())
}
