pub use alt::prelude::*;

#[alt::main(crate_name = "alt")]
pub fn main() {
    // TODO: fix backtrace panic
    std::env::set_var("RUST_BACKTRACE", "full");

    alt::events::on_connection_queue_add(|c| {
        alt::log!("on_connection_queue_add");
        dbg!(c.info());
        let controller = c.controller();
        alt::set_timeout(
            move || {
                dbg!(controller.decline("лох"));
                Ok(())
            },
            3000,
        );
        Ok(())
    });

    alt::events::on_connection_queue_remove(|c| {
        alt::log!("on_connection_queue_remove");
        dbg!(c.info());
        Ok(())
    });

    alt::events::on_global_meta_change(|c| {
        dbg!(c);
        Ok(())
    });

    alt::events::on_global_synced_meta_change(|c| {
        dbg!(c);
        Ok(())
    });

    alt::events::on_synced_meta_change(|c| {
        dbg!(c);
        Ok(())
    });

    alt::events::on_stream_synced_meta_change(|c| {
        dbg!(c);
        Ok(())
    });

    alt::events::on_local_meta_change(|c| {
        dbg!(c);
        Ok(())
    });
}
