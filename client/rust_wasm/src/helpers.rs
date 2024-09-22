use std::time::Duration;

use crate::wasm_imports;

// TODO: add NetTime struct similar to std Instant
pub fn net_time() -> Duration {
  Duration::from_millis(wasm_imports::get_net_time().into())
}
