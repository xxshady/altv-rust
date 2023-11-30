use std::time::{Instant, Duration};

#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    let now = Instant::now();
    altv::spawn_async(async move {
        for _ in 1..=3 {
            altv::dbg!(now.elapsed());
            altv::wait(Duration::from_secs(1)).await;
        }
    })
    .unwrap();
}
