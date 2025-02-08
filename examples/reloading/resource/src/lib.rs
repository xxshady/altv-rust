#[altv::main]
fn main() -> impl altv::IntoVoidResult {
  altv::log!("~gl~hello world");

  altv::log!("bt:\n{:#}", std::backtrace::Backtrace::force_capture());

  // altv::anyhow::bail!("this is an error");
}
