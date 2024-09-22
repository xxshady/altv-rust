use std::fmt::Debug;

use crate::wasm_imports::{self, BaseObject as JsBaseObjectRef};
use super::{
  base_object_js_ref::BaseObjectJsRef, as_base_object_type::AsBaseObjectType,
  handle::BaseObjectHandle, manager::Manager,
};

#[derive(Clone)]
pub struct BaseObject<T: AsBaseObjectType> {
  pub(crate) handle: BaseObjectHandle<T>,
  pub(crate) js_ref: JsBaseObjectRef,
}

impl<T: AsBaseObjectType> BaseObject<T> {
  pub(crate) fn new(handle: BaseObjectHandle<T>, js_ref: JsBaseObjectRef) -> Self {
    Self { handle, js_ref }
  }

  pub(crate) fn new_by_handle(manager: &Manager, handle: BaseObjectHandle<T>) -> Option<Self> {
    let base_handle = handle.as_generic();
    let valid = manager.is_handle_valid(&base_handle);
    if valid {
      let js_ref = base_handle.js_ref();
      Some(Self { handle, js_ref })
    } else {
      None
    }
  }

  /// See [`BaseObjectHandle`](super::handle::BaseObjectHandle).
  pub fn handle(&self) -> BaseObjectHandle<T> {
    self.handle
  }
}

impl<T: AsBaseObjectType> BaseObjectJsRef for BaseObject<T> {
  fn js_ref(&self) -> &wasm_imports::BaseObject {
    &self.js_ref
  }
}

impl<T: AsBaseObjectType> Debug for BaseObject<T> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let handle = self.handle.as_generic();
    write!(
      f,
      "{:?} {{ id: {}, generation: {} }}",
      handle.btype, handle.id, handle.generation
    )
  }
}
