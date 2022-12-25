#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[alt::res_main]
#[no_mangle]
pub fn main() {
    alt::log("test ~r~red ~bl~yes ~wl~fuck yea");

    // let model_name = "sultan3";
    // let hash = alt::hash(model_name);
    // alt::log(&format!(
    //     "creating vehicle of model: {model_name} hash: {hash}"
    // ));

    // let veh = alt::create_vehicle(hash, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    // alt::log(&format!("created veh: {}", veh.id()));
    // std::thread::spawn(move || {
    //     // alt::log("wait for 4000ms");
    //     println!("wait for 4000ms");
    //     // std::thread::sleep(std::time::Duration::from_millis(4000));
    //     // alt::log(&format!("destroy veh"));
    //     // veh.destroy();
    // });
}
