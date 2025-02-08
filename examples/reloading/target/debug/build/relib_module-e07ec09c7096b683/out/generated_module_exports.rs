// This file is generated, DO NOT edit manually
// ---------------------------------------------

use std::ffi::c_void;
use relib_internal_shared::{ModuleId, SliceAllocation};
use relib_internal_shared::exports::___Internal___Exports___ as Exports;
/// Struct for implementing your `Exports` trait
pub struct ModuleExportsImpl;
#[unsafe(export_name = "__relib_____Internal___Exports____init")]
pub extern "C" fn init(host_owner_thread: usize, module: ModuleId) {
    <ModuleExportsImpl as Exports>::init(host_owner_thread, module)
}
#[unsafe(export_name = "__relib_____Internal___Exports____exit")]
pub extern "C" fn exit(allocs: SliceAllocation) {
    <ModuleExportsImpl as Exports>::exit(allocs)
}
#[unsafe(
    export_name = "__relib_____Internal___Exports____take_cached_allocs_before_exit"
)]
pub extern "C" fn take_cached_allocs_before_exit() {
    <ModuleExportsImpl as Exports>::take_cached_allocs_before_exit()
}
#[unsafe(export_name = "__relib_____Internal___Exports____lock_module_allocator")]
pub extern "C" fn lock_module_allocator() {
    <ModuleExportsImpl as Exports>::lock_module_allocator()
}
#[unsafe(export_name = "__relib_____Internal___Exports____spawned_threads_count")]
pub extern "C" fn spawned_threads_count() -> u64 {
    <ModuleExportsImpl as Exports>::spawned_threads_count()
}
#[unsafe(export_name = "__relib_____Internal___Exports____run_thread_local_dtors")]
pub extern "C" fn run_thread_local_dtors() {
    <ModuleExportsImpl as Exports>::run_thread_local_dtors()
}
#[unsafe(export_name = "__relib_____Internal___Exports____misc_cleanup")]
pub extern "C" fn misc_cleanup() {
    <ModuleExportsImpl as Exports>::misc_cleanup()
}
#[unsafe(export_name = "__relib_____Internal___Exports____set_dealloc_callback")]
pub extern "C" fn set_dealloc_callback(callback: *const c_void) {
    <ModuleExportsImpl as Exports>::set_dealloc_callback(callback)
}
