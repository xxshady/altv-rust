use crate::base_objects::base_object_js_ref::BaseObjectJsRef;

pub trait Vehicle: BaseObjectJsRef {
  fn fuel_level(&self) -> f32 {
    self.js_ref().fuel_level()
  }
  fn set_fuel_level(&self, value: f32) {
    self.js_ref().set_fuel_level(value);
  }
}
