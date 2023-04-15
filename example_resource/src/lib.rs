pub use alt::prelude::*;
use rand::Rng;

#[alt::main(crate_name = "alt")]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");

    alt::events::on_player_connect(|c| {
        let mut i = 0;
        let player = c.player.clone();
        let mut rng = rand::thread_rng();
        alt::set_interval(
            move || {
                for _ in 0..100 {
                    alt::events::emit_client!(
                        "test",
                        player.clone(),
                        i as i64,
                        "d".repeat(rng.gen_range(1000..100_000)).as_str()
                    )?;
                    i += 1;
                }
                Ok(())
            },
            0,
        );
        Ok(())
    });
}
