pub type VoidResult = anyhow::Result<()>;
pub type SomeResult<V> = anyhow::Result<V>;

pub trait IntoVoidResult {
    fn into_void_result(self) -> VoidResult;
}

impl IntoVoidResult for VoidResult {
    fn into_void_result(self) -> VoidResult {
        self
    }
}

impl IntoVoidResult for () {
    fn into_void_result(self) -> VoidResult {
        Ok(())
    }
}
