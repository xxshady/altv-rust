use crate::base_objects::base_object_js_ref::BaseObjectJsRef;

pub trait Player: BaseObjectJsRef {
  fn name(&self) -> String {
    self.js_ref().name()
  }
}
