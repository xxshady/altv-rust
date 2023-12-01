use crate::{ffi, BaseObjectType};

// why there is no NonNull for const pointers? :c
pub unsafe fn get_base_object_type(base_object: *const ffi::alt::IBaseObject) -> BaseObjectType {
    let raw_type = unsafe { ffi::IBaseObject::GetType(base_object) };
    BaseObjectType::try_from(raw_type)
        .expect("failed to convert raw baseobj type: {raw_type} to BaseObjectType")
}

#[macro_export]
macro_rules! __impl_extern_type_callback {
    ($name:ident, $type_id_name:literal) => {
        unsafe impl ExternType for $name {
            type Id = type_id!($type_id_name);
            type Kind = cxx::kind::Trivial;
        }
    };
}
pub(crate) use __impl_extern_type_callback as impl_extern_type_callback;

pub fn log(str: &str) {
    unsafe {
        ffi::ICore::LogColored(str, std::ptr::null_mut());
    }
}

pub unsafe fn log_error(str: &str) {
    unsafe {
        ffi::ICore::LogError(str, std::ptr::null_mut());
    }
}

pub unsafe fn log_warning(str: &str) {
    unsafe {
        ffi::ICore::LogWarning(str, std::ptr::null_mut());
    }
}

pub unsafe fn log_error_with_resource(str: &str, resource: *mut ffi::alt::IResource) {
    unsafe {
        ffi::ICore::LogError(str, resource);
    }
}

pub unsafe fn log_with_resource(str: &str, resource: *mut ffi::alt::IResource) {
    unsafe {
        ffi::ICore::LogColored(str, resource);
    }
}

pub unsafe fn log_warning_with_resource(str: &str, resource: *mut ffi::alt::IResource) {
    unsafe {
        ffi::ICore::LogWarning(str, resource);
    }
}
