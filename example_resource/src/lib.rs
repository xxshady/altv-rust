use altv::prelude::*;
#[altv::main]
fn main() -> impl altv::IntoVoidResult {
    std::env::set_var("RUST_BACKTRACE", "full");

    altv::events::on("example", |event| {
        let args: (bool, i32) = event.args.deserialize()?;
        altv::log!("args: {args:?}"); // a(true, 123)
        Ok(())
    });

    altv::events::emit("example", &[&true, &123])?;

    Ok(())
}
