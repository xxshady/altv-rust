use altv::anyhow::anyhow;

pub(crate) fn test_error_backtrace() {
  let error = anyhow!("test error");
  altv::log!("{}", error.backtrace());
}
