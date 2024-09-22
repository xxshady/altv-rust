use altv_sdk::ffi as sdk;

const PREFIX: &str = "[rust]";

pub fn log(str: &str) {
  unsafe {
    sdk::ICore::LogColored(PREFIX, str, std::ptr::null_mut());
  }
}

pub fn log_error(str: &str) {
  unsafe {
    sdk::ICore::LogError(PREFIX, str, std::ptr::null_mut());
  }
}

pub fn log_warn(str: &str) {
  unsafe {
    sdk::ICore::LogWarning(PREFIX, str, std::ptr::null_mut());
  }
}
