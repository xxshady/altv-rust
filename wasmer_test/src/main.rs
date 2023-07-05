extern "C" {
    fn altv_log(message: &str);
}

fn main() {
    unsafe { altv_log("test") }
}
