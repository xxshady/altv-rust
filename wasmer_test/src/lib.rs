extern "C" {
    fn fibo_end();
}

#[no_mangle]
extern "C" fn main() {
    for i in 1..40 {
        fibo(i);
    }
    unsafe { fibo_end() }
}

fn fibo(n: i32) -> i32 {
    if n < 2 {
        1
    } else {
        fibo(n - 2) + fibo(n - 1)
    }
}
