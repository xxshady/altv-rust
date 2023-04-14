pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    alt::events::on_any_resource_start(|c| {
        alt::log!("some resource: {} ~gl~start", c.resource.name);
        Ok(())
    });

    alt::events::on_any_resource_stop(|c| {
        alt::log!("some resource: {} ~rl~stop", c.resource.name);
        Ok(())
    });

    alt::events::on_resource_start(|_| {
        alt::log!("this resource ~gl~start");
        Ok(())
    });

    alt::events::on_resource_stop(|_| {
        alt::log!("this resource ~rl~stop");
        Ok(())
    });

    let resource = alt::Resource::current();

    alt::set_timeout(
        move || {
            dbg!(alt::Player::all());
            resource.restart();
            Ok(())
        },
        1000,
    );
}
