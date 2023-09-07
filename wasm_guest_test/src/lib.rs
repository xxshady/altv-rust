#[no_mangle]
extern "C" fn main() {
    altv::log!("~gl~start!");

    altv::natives::do_screen_fade_out(0);
    altv::natives::do_screen_fade_in(3000);

    let mut screen_in = true;
    altv::set_interval(
        move || {
            screen_in = !screen_in;
            if screen_in {
                altv::natives::do_screen_fade_out(2000);
            } else {
                altv::natives::do_screen_fade_in(2000);
            }
        },
        2000,
    );
}
