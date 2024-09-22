use crate::any_error::AnyError;

pub type AnyVoidResult = Result<(), AnyError>;

pub trait IntoAnyVoidResult {
  fn into_any_void_result(self) -> AnyVoidResult;
}

impl IntoAnyVoidResult for () {
  fn into_any_void_result(self) -> AnyVoidResult {
    Ok(())
  }
}

impl IntoAnyVoidResult for Result<(), AnyError> {
  fn into_any_void_result(self) -> AnyVoidResult {
    self
  }
}
