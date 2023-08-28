#[macro_export]
macro_rules! __assert_its_called_once {
    () => {{
        use std::cell::Cell;

        thread_local! {
            static CALLED: Cell<bool> = Cell::new(false);
        }

        CALLED.with(|v| {
            assert!(!v.get(), "Cannot be called multiple times");
            v.set(true);
        });
    }};
}

pub use __assert_its_called_once as assert_its_called_once;
