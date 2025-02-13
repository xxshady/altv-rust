use altv_sdk::EventType;
use crate::abi_stable::Str;

pub trait Imports {
  fn toggle_event_type(resource: Str, ty: EventType, enable: bool);
}
