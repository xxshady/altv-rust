use std::time::{Instant, Duration};

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    let now = Instant::now();
    altv::spawn_async(async move {
        for _ in 1..=3 {
            altv::futures::join!(
                altv::wait(Duration::from_secs(1)),
                altv::wait(Duration::from_secs(2)),
                altv::wait(Duration::from_millis(1500))
            );
            altv::dbg!(now.elapsed());
        }
    })
    .unwrap();
}
