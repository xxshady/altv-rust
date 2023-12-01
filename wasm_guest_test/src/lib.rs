use std::time::{Instant, Duration};

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    altv::spawn_async(async move {
        let now: Instant = Instant::now();
        altv::wait_for(
            || {
                if now.elapsed() >= Duration::from_millis(1500) {
                    true
                } else {
                    false
                }
            },
            Duration::from_secs(1),
        )
        .await?;

        altv::dbg!(now.elapsed());

        Ok(())
    })
    .unwrap();
}
