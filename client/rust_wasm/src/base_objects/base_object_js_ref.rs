use crate::wasm_imports;

pub trait BaseObjectJsRef {
  fn js_ref(&self) -> &wasm_imports::BaseObject;
}
