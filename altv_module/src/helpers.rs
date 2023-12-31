use altv_sdk::ffi as sdk;
use altv_wasm_shared::{BaseObjectPtr, BaseObjectWithType, BaseObjectType};

use crate::resource_manager::{RESOURCE_MANAGER_INSTANCE, is_pending_base_object};

pub fn handle_base_object_creation_or_deletion(
    resource_name: &str,
    base_object: altv_sdk::BaseObjectRawMutPtr,
    creation: bool,
) {
    RESOURCE_MANAGER_INSTANCE.with(|v| {
        let manager = v.borrow();

        if is_pending_base_object() {
            logger::debug!("pending_base_object, ignoring");
            return;
        }

        if manager.is_resource_starting(resource_name) {
            logger::debug!("Resource is starting, ignoring");
            return;
        }

        let Some(r) = manager.get_by_name(resource_name) else {
            logger::error!("Unknown resource: {resource_name:?}");
            return;
        };

        let ty = unsafe { altv_sdk::ffi::IBaseObject::GetType(base_object) };

        // ignoring because error will be printed in call_export
        let _ = r.call_export(|e| {
            if creation {
                e.call_on_base_object_create(base_object as u64, ty)
            } else {
                e.call_on_base_object_destroy(base_object as u64)
            }
        });
    });
}

pub unsafe fn serialize_base_object_for_event<T>(
    ptr: *mut T,
    to_base: unsafe fn(*mut T) -> altv_sdk::BaseObjectRawMutPtr,
) -> BaseObjectWithType {
    let base_ptr = to_base(ptr);
    let ty = sdk::IBaseObject::GetType(base_ptr);

    (
        base_ptr as BaseObjectPtr,
        BaseObjectType::try_from(ty).unwrap(),
    )
}
