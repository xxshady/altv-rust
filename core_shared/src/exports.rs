use altv_sdk::{
  ffi::alt::ICore, BaseObjectMutPtr, BaseObjectType, CEventPtr, EventType as SDKEventType,
};

use crate::abi_stable::{OwnedStr, Str};

pub trait Exports {
  fn init(core_ptr: *mut ICore, resource_name: Str);

  fn altv_crate_version() -> OwnedStr;

  fn on_tick();
  fn on_base_object_create(base_object: BaseObjectMutPtr, ty: BaseObjectType);
  fn on_base_object_destroy(base_object: BaseObjectMutPtr, ty: BaseObjectType);
  fn on_sdk_event(sdk_event_type: SDKEventType, event: CEventPtr);
}
