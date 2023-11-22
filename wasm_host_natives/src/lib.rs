use std::cell::RefCell;
use altv_wasm_shared::natives_result::*;
use altv_sdk::ffi as sdk;
use autocxx::prelude::*;
use memory_buffer::MemoryBufferManager;

mod memory_buffer;

pub struct WasmNatives {
    pub memory_buffers: RefCell<MemoryBufferManager>,
}

impl WasmNatives {
    pub fn new() -> Self {
        Self {
            memory_buffers: RefCell::new(MemoryBufferManager::new()),
        }
    }
}

impl wasm_host::gen::imports::extra_interfaces::wasm_natives for WasmNatives {
    fn native_get_dlc_weapon_data(&self, dlcWeaponIndex_: i32,
outData_: shared::MemoryBufferId) -> ResultOf_get_dlc_weapon_data {
    unsafe {
        let mut native_return = Default::default();
let mut dlcWeaponIndex_ = dlcWeaponIndex_;
let mut outData_ = self.memory_buffers.borrow_mut().get_mut_ptr(outData_) as *mut c_void;
        let success = sdk::natives::get_dlc_weapon_data(
            &mut native_return,
outData_,
dlcWeaponIndex_,
        );
        let native_return = native_return;
let outData_ = 0;
        
        ResultOf_get_dlc_weapon_data {
            success,
            ret: native_return,
            outData_
        }
    }
}

}
