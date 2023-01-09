use alt::resource_api::TestData;
use std::sync::{Arc, Mutex};

#[no_mangle]
extern "C" fn test_interval_c() {
    println!("test_interval_c called");
}

fn test_interval() {
    println!("test_interval called");
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[alt::res_main]
#[no_mangle]
pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let resource = alt::MainResource::new(full_main_path, resource_api);

    let callback = Arc::new(Mutex::new(alt::log as fn(&str)));

    let test_data = Arc::new(Mutex::new(TestData {
        a: 10,
        callback: callback.clone(),
    }));

    alt::set_interval(test_interval, 1000, test_data.clone());

    test_data.try_lock().unwrap().a += 1;
    // (callback.try_lock().unwrap())();

    resource
}
