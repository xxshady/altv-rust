use std::cell::RefCell;
use wasm_bindgen::prelude::*;

thread_local! {
  pub static CURRENT_PANIC_INFO: RefCell<Option<String>> = Default::default();
}

#[wasm_bindgen]
pub fn get_current_panic_info() -> Option<String> {
  CURRENT_PANIC_INFO.take()
}

pub fn init() {
  std::panic::set_hook(Box::new(|info| {
    CURRENT_PANIC_INFO.replace(Some(info.to_string()));
  }));
}
