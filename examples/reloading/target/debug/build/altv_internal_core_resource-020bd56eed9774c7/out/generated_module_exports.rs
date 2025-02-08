// This file is generated, DO NOT edit manually
// ---------------------------------------------

use altv_sdk::{
    ffi::alt::ICore, BaseObjectMutPtr, BaseObjectType, CEventPtr,
    EventType as SDKEventType,
};
use core_shared::abi_stable::{OwnedStr, Str};
use core_shared::exports::Exports as Exports;
/// Struct for implementing your `Exports` trait
pub struct ModuleExportsImpl;
#[unsafe(no_mangle)]
#[allow(clippy::needless_lifetimes)]
pub extern "C" fn __relib__Exports_init(
    ____success____: *mut bool,
    core_ptr: *mut ICore,
    resource_name: Str,
) -> std::mem::MaybeUninit<()> {
    let result = std::panic::catch_unwind(move || {
        <ModuleExportsImpl as Exports>::init(core_ptr, resource_name)
    });
    match result {
        Ok(return_value) => {
            unsafe {
                *____success____ = true;
            }
            #[allow(unused_braces, clippy::unit_arg)]
            std::mem::MaybeUninit::new({ return_value })
        }
        Err(_) => {
            unsafe {
                *____success____ = false;
            }
            std::mem::MaybeUninit::uninit()
        }
    }
}
#[unsafe(no_mangle)]
#[allow(clippy::needless_lifetimes)]
pub extern "C" fn __relib__Exports_altv_crate_version(
    ____success____: *mut bool,
) -> std::mem::MaybeUninit<*mut OwnedStr> {
    let result = std::panic::catch_unwind(move || {
        <ModuleExportsImpl as Exports>::altv_crate_version()
    });
    match result {
        Ok(return_value) => {
            unsafe {
                *____success____ = true;
            }
            #[allow(unused_braces, clippy::unit_arg)]
            std::mem::MaybeUninit::new({
                use std::boxed::Box;
                Box::into_raw(Box::new(return_value))
            })
        }
        Err(_) => {
            unsafe {
                *____success____ = false;
            }
            std::mem::MaybeUninit::uninit()
        }
    }
}
#[unsafe(no_mangle)]
#[allow(clippy::needless_lifetimes, clippy::extra_unused_lifetimes)]
pub extern "C" fn __post__relib__Exports_altv_crate_version(
    return_value_ptr: *mut OwnedStr,
) {
    use std::boxed::Box;
    unsafe {
        drop(Box::from_raw(return_value_ptr));
    }
}
#[unsafe(no_mangle)]
#[allow(clippy::needless_lifetimes)]
pub extern "C" fn __relib__Exports_on_tick(
    ____success____: *mut bool,
) -> std::mem::MaybeUninit<()> {
    let result = std::panic::catch_unwind(move || {
        <ModuleExportsImpl as Exports>::on_tick()
    });
    match result {
        Ok(return_value) => {
            unsafe {
                *____success____ = true;
            }
            #[allow(unused_braces, clippy::unit_arg)]
            std::mem::MaybeUninit::new({ return_value })
        }
        Err(_) => {
            unsafe {
                *____success____ = false;
            }
            std::mem::MaybeUninit::uninit()
        }
    }
}
#[unsafe(no_mangle)]
#[allow(clippy::needless_lifetimes)]
pub extern "C" fn __relib__Exports_on_base_object_create(
    ____success____: *mut bool,
    base_object: BaseObjectMutPtr,
    ty: BaseObjectType,
) -> std::mem::MaybeUninit<()> {
    let result = std::panic::catch_unwind(move || {
        <ModuleExportsImpl as Exports>::on_base_object_create(base_object, ty)
    });
    match result {
        Ok(return_value) => {
            unsafe {
                *____success____ = true;
            }
            #[allow(unused_braces, clippy::unit_arg)]
            std::mem::MaybeUninit::new({ return_value })
        }
        Err(_) => {
            unsafe {
                *____success____ = false;
            }
            std::mem::MaybeUninit::uninit()
        }
    }
}
#[unsafe(no_mangle)]
#[allow(clippy::needless_lifetimes)]
pub extern "C" fn __relib__Exports_on_base_object_destroy(
    ____success____: *mut bool,
    base_object: BaseObjectMutPtr,
    ty: BaseObjectType,
) -> std::mem::MaybeUninit<()> {
    let result = std::panic::catch_unwind(move || {
        <ModuleExportsImpl as Exports>::on_base_object_destroy(base_object, ty)
    });
    match result {
        Ok(return_value) => {
            unsafe {
                *____success____ = true;
            }
            #[allow(unused_braces, clippy::unit_arg)]
            std::mem::MaybeUninit::new({ return_value })
        }
        Err(_) => {
            unsafe {
                *____success____ = false;
            }
            std::mem::MaybeUninit::uninit()
        }
    }
}
#[unsafe(no_mangle)]
#[allow(clippy::needless_lifetimes)]
pub extern "C" fn __relib__Exports_on_sdk_event(
    ____success____: *mut bool,
    sdk_event_type: SDKEventType,
    event: CEventPtr,
) -> std::mem::MaybeUninit<()> {
    let result = std::panic::catch_unwind(move || {
        <ModuleExportsImpl as Exports>::on_sdk_event(sdk_event_type, event)
    });
    match result {
        Ok(return_value) => {
            unsafe {
                *____success____ = true;
            }
            #[allow(unused_braces, clippy::unit_arg)]
            std::mem::MaybeUninit::new({ return_value })
        }
        Err(_) => {
            unsafe {
                *____success____ = false;
            }
            std::mem::MaybeUninit::uninit()
        }
    }
}
