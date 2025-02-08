// This file is generated, DO NOT edit manually
// ---------------------------------------------

use relib_internal_shared::{ModuleId, SliceAllocatorOp, StableLayout, Str};
/**# Safety
Behavior is undefined if any of the following conditions are violated:
1. Types of arguments and return value must be FFI-safe.
2. Host and module crates must be compiled with same shared crate code (which contains exports and imports traits).
3. Returned value must not be a reference-counting pointer (see [caveats](https://docs.rs/relib/latest/relib/#moving-non-copy-types-between-host-and-module)).*/
pub unsafe fn on_alloc(module: ModuleId, ptr: *mut u8, layout: StableLayout) {
    #[allow(non_upper_case_globals)]
    #[unsafe(no_mangle)]
    static mut __relib_____Internal___Imports____on_alloc: extern "C" fn(
        module: ModuleId,
        ptr: *mut u8,
        layout: StableLayout,
    ) = placeholder;
    extern "C" fn placeholder(_: ModuleId, _: *mut u8, _: StableLayout) {
        unreachable!();
    }
    unsafe { __relib_____Internal___Imports____on_alloc(module, ptr, layout) }
}
/**# Safety
Behavior is undefined if any of the following conditions are violated:
1. Types of arguments and return value must be FFI-safe.
2. Host and module crates must be compiled with same shared crate code (which contains exports and imports traits).
3. Returned value must not be a reference-counting pointer (see [caveats](https://docs.rs/relib/latest/relib/#moving-non-copy-types-between-host-and-module)).*/
pub unsafe fn on_cached_allocs(module: ModuleId, ops: SliceAllocatorOp) {
    #[allow(non_upper_case_globals)]
    #[unsafe(no_mangle)]
    static mut __relib_____Internal___Imports____on_cached_allocs: extern "C" fn(
        module: ModuleId,
        ops: SliceAllocatorOp,
    ) = placeholder;
    extern "C" fn placeholder(_: ModuleId, _: SliceAllocatorOp) {
        unreachable!();
    }
    unsafe { __relib_____Internal___Imports____on_cached_allocs(module, ops) }
}
/**# Safety
Behavior is undefined if any of the following conditions are violated:
1. Types of arguments and return value must be FFI-safe.
2. Host and module crates must be compiled with same shared crate code (which contains exports and imports traits).
3. Returned value must not be a reference-counting pointer (see [caveats](https://docs.rs/relib/latest/relib/#moving-non-copy-types-between-host-and-module)).*/
pub unsafe fn unrecoverable(message: Str) -> ! {
    #[allow(non_upper_case_globals)]
    #[unsafe(no_mangle)]
    static mut __relib_____Internal___Imports____unrecoverable: extern "C" fn(
        message: Str,
    ) -> ! = placeholder;
    extern "C" fn placeholder(_: Str) -> ! {
        unreachable!();
    }
    unsafe { __relib_____Internal___Imports____unrecoverable(message) }
}
