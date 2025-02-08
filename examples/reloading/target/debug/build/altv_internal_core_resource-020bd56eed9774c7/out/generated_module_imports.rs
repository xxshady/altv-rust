// This file is generated, DO NOT edit manually
// ---------------------------------------------

use altv_sdk::EventType;
use core_shared::abi_stable::Str;
/**# Safety
Behavior is undefined if any of the following conditions are violated:
1. Types of arguments and return value must be FFI-safe.
2. Host and module crates must be compiled with same shared crate code (which contains exports and imports traits).
3. Returned value must not be a reference-counting pointer (see [caveats](https://docs.rs/relib/latest/relib/#moving-non-copy-types-between-host-and-module)).*/
pub unsafe fn toggle_event_type(resource: Str, ty: EventType, enable: bool) {
    #[allow(non_upper_case_globals)]
    #[unsafe(no_mangle)]
    static mut __relib__Imports_toggle_event_type: extern "C" fn(
        ____success____: *mut bool,
        resource: Str,
        ty: EventType,
        enable: bool,
    ) -> std::mem::MaybeUninit<()> = ____placeholder____;
    #[allow(clippy::needless_lifetimes)]
    extern "C" fn ____placeholder____(
        _: *mut bool,
        _: Str,
        _: EventType,
        _: bool,
    ) -> std::mem::MaybeUninit<()> {
        unreachable!();
    }
    let mut ____success____ = std::mem::MaybeUninit::<bool>::uninit();
    #[allow(unused_variables, clippy::let_unit_value, clippy::diverging_sub_expression)]
    let return_value = unsafe {
        __relib__Imports_toggle_event_type(
            ____success____.as_mut_ptr(),
            resource,
            ty,
            enable,
        )
    };
    if !____success____.assume_init() {
        eprintln!(
            "[relib] host panicked while executing import {:?} of module, aborting",
            stringify!(toggle_event_type)
        );
        std::process::abort();
    }
    return_value.assume_init()
}
