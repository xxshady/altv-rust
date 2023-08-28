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
        if creation {
            r.wasi_exports
                .borrow_mut()
                .call_on_base_object_create(base_object as u64, ty)
                .unwrap_or_else(|e| {
                    logger::error!("call_on_base_object_create failed: {e:?}");
                });
        } else {
            r.wasi_exports
                .borrow_mut()
                .call_on_base_object_destroy(base_object as u64, ty)
                .unwrap_or_else(|e| {
                    logger::error!("call_on_base_object_destroy failed: {e:?}");
                });
        }
    });
}
