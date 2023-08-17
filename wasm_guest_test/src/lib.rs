#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~main");

    let mut i = 0;
    altv::set_interval(
        move || {
            i += 1;
            altv::dbg!(i);
            Ok(())
        },
        1500,
    );
}
