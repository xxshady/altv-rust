use crate::{ffi, BaseObjectType};

// why there is no NonNull for const pointers? :c
pub unsafe fn get_base_object_type(base_object: *const ffi::alt::IBaseObject) -> BaseObjectType {
    let raw_type = unsafe { ffi::IBaseObject::GetType(base_object) };
    BaseObjectType::from(raw_type)
        .expect("failed to convert raw baseobj type: {raw_type} to BaseObjectType")
}
