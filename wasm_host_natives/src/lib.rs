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

impl wasm_host::gen::imports::extra_interfaces::WasmNatives for WasmNatives {
    fn native_get_dlc_weapon_data(
        &self,
        dlc_weapon_index_: i32,
        out_data_: shared::MemoryBufferId,
    ) -> ResultOfGetDlcWeaponData {
        unsafe {
            let mut native_return = Default::default();
            let mut dlc_weapon_index_ = dlc_weapon_index_;
            let mut out_data_ =
                self.memory_buffers.borrow_mut().get_mut_ptr(out_data_) as *mut c_void;
            let success =
                sdk::natives::get_dlc_weapon_data(&mut native_return, out_data_, dlc_weapon_index_);
            let native_return = native_return;
            let out_data_ = 0;

            ResultOfGetDlcWeaponData {
                success,
                ret: native_return,
                out_data_,
            }
        }
    }
}
