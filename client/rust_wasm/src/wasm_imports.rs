use js_sys::ArrayBuffer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_namespace = altv_imports)]
extern "C" {
  #[wasm_bindgen]
  pub fn log_info(data: &str);

  #[wasm_bindgen]
  pub fn log_warn(data: &str);

  #[wasm_bindgen]
  pub fn log_error(data: &str);

  // TODO: dont send event as string for better performance
  #[wasm_bindgen]
  pub fn enable_altv_event(event_name: &str);
  #[wasm_bindgen]
  pub fn disable_altv_event(event_name: &str);

  #[wasm_bindgen]
  pub fn get_base_object_ref(sdk_type: u8, is_remote: bool, id: u32) -> Option<BaseObject>;

  #[derive(Clone)]
  pub type BaseObject;

  #[wasm_bindgen(method, getter)]
  pub fn id(this: &BaseObject) -> u32;

  #[wasm_bindgen(method, getter, js_name = "isRemote")]
  pub fn is_remote(this: &BaseObject) -> bool;

  #[wasm_bindgen(method)]
  pub fn destroy(this: &BaseObject);

  // world object
  #[wasm_bindgen(method, getter)]
  pub fn pos(this: &BaseObject) -> JsValue;
  #[wasm_bindgen(method, setter)]
  pub fn set_pos(this: &BaseObject, value: JsValue);

  #[wasm_bindgen(method, getter)]
  pub fn dimension(this: &BaseObject) -> i32;
  #[wasm_bindgen(method, setter)]
  pub fn set_dimension(this: &BaseObject, value: i32);

  // entity
  #[wasm_bindgen(method, getter)]
  pub fn model(this: &BaseObject) -> u32;

  #[wasm_bindgen(method, getter, js_name = "netOwner")]
  pub fn net_owner(this: &BaseObject) -> Option<BaseObject>;

  // player
  #[wasm_bindgen(method, getter)]
  pub fn name(this: &BaseObject) -> String;

  // vehicle
  #[wasm_bindgen(method, getter, js_name = "fuelLevel")]
  pub fn fuel_level(this: &BaseObject) -> f32;
  #[wasm_bindgen(method, setter, js_name = "fuelLevel")]
  pub fn set_fuel_level(this: &BaseObject, value: f32);

  // local player
  #[wasm_bindgen(method, getter, js_name = "currentAmmo")]
  pub fn current_ammo(this: &BaseObject) -> u16;

  #[wasm_bindgen]
  pub fn emit_local_event_rust(event_name: &str, buffer: ArrayBuffer);

  #[wasm_bindgen]
  pub fn emit_local_event_js(event_name: &str, args: JsValue);

  // returns Vec<PlayerHandle>
  #[wasm_bindgen]
  pub fn get_streamed_in_players() -> JsValue;

  #[wasm_bindgen]
  pub fn get_streamed_in_vehicles() -> JsValue;

  #[wasm_bindgen]
  pub fn get_net_time() -> u32;

  #[wasm_bindgen]
  pub fn get_base_object_raw_handle(js_ref: &BaseObject) -> JsValue;

  #[wasm_bindgen]
  pub fn get_local_player() -> BaseObject;

  #[wasm_bindgen]
  pub fn is_local_player(base_object: &BaseObject) -> bool;
}

#[wasm_bindgen]
extern "C" {
  pub type Error;

  #[wasm_bindgen(constructor)]
  pub fn new() -> Error;

  #[wasm_bindgen(structural, method, getter)]
  pub fn stack(error: &Error) -> String;
}
