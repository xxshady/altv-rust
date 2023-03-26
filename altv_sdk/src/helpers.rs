use crate::{ffi, BaseObjectType};

pub unsafe fn get_base_object_type(base_object: *const ffi::alt::IBaseObject) -> BaseObjectType {
    let raw_type = unsafe { ffi::IBaseObject::GetType(base_object) };
    if raw_type == 255 {
        panic!("resource_on_create_base_object base_object type is invalid");
    }

    BaseObjectType::from(raw_type)
        .expect("failed to convert raw baseobj type: {raw_type} to BaseObjectType")
}
