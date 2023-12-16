use autocxx::prelude::*;
use wasmtime_wasi::WasiCtx;
use altv_sdk::ffi as sdk;
use sdk_helpers::read_cpp_vector3;
use shared::{Vector3, MemoryBufferId};
use crate::resource_manager::set_pending_base_object;

pub use wasm_host::gen::imports;

pub type Exports = wasm_host::gen::exports::Exports<State>;

pub struct State {
    pub wasi: WasiCtx,

    // TODO: some safe wrapper over this unsafe shit
    pub resource_ptr: *mut sdk::shared::AltResource,
    pub big_call_ptr: wasm_host::gen::Ptr,

    memory: Option<wasmtime::Memory>,
    free: Option<wasm_host::gen::FreeFunc>,
    alloc: Option<wasm_host::gen::AllocFunc>,
    natives: wasm_host_natives::WasmNatives,
}

impl State {
    pub fn new(wasi: WasiCtx, resource_ptr: *mut sdk::shared::AltResource) -> Self {
        Self {
            wasi,
            resource_ptr,
            memory: None,
            free: None,
            alloc: None,
            big_call_ptr: 0,
            natives: wasm_host_natives::WasmNatives::new(),
        }
    }
}

macro_rules! base_ptr_as {
    ($to:ident, $ptr:expr) => {
        paste::paste! { {
            let to = sdk::base_object::[<to_ $to>]($ptr as _);
            assert!(!to.is_null());
            to
        } }
    };
}

impl wasm_host::gen::imports::Imports for State {
    type ExtraInterfaceWasmNatives = wasm_host_natives::WasmNatives;

    fn get_wasm_natives(&self) -> &Self::ExtraInterfaceWasmNatives {
        &self.natives
    }

    fn get_big_call_ptr(&self) -> u32 {
        self.big_call_ptr
    }

    fn get_memory(&self) -> Option<wasmtime::Memory> {
        self.memory
    }

    fn get_free(&self) -> Option<wasm_host::gen::FreeFunc> {
        self.free
    }

    fn get_alloc(&self) -> Option<wasm_host::gen::AllocFunc> {
        self.alloc
    }

    fn set_memory(&mut self, memory: wasmtime::Memory) {
        self.memory.replace(memory);
    }

    fn set_free(&mut self, value: wasm_host::gen::FreeFunc) {
        self.free.replace(value);
    }

    fn set_alloc(&mut self, alloc: wasm_host::gen::AllocFunc) {
        self.alloc.replace(alloc);
    }

    fn log_error(&self, message: String) {
        unsafe { altv_sdk::helpers::log_error_with_resource(&message, self.resource_ptr) }
    }

    fn log_warn(&self, message: String) {
        unsafe { altv_sdk::helpers::log_warning_with_resource(&message, self.resource_ptr) }
    }

    fn log(&self, message: String) {
        unsafe { altv_sdk::helpers::log_with_resource(&message, self.resource_ptr) }
    }

    fn destroy_base_object(&self, ptr: altv_wasm_shared::BaseObjectPtr) {
        set_pending_base_object(true);
        unsafe { sdk::ICore::DestroyBaseObject(ptr as _) }
        set_pending_base_object(false);
    }

    fn base_object_get_id(&self, ptr: altv_wasm_shared::BaseObjectPtr) -> u32 {
        unsafe { sdk::IBaseObject::GetID(ptr as _) }
    }

    fn base_object_get_remote_id(&self, ptr: altv_wasm_shared::BaseObjectPtr) -> u32 {
        unsafe { sdk::IBaseObject::GetRemoteID(ptr as _) }
    }

    fn world_object_get_pos(&self, ptr: altv_wasm_shared::BaseObjectPtr) -> Vector3 {
        read_cpp_vector3(
            unsafe { sdk::IWorldObject::GetPosition(base_ptr_as!(world_object, ptr)) }
                .within_unique_ptr(),
        )
    }

    fn world_object_set_pos(
        &self,
        ptr: altv_wasm_shared::BaseObjectPtr,
        // TODO: maybe use three parameters instead for better performance?
        value: Vector3,
    ) {
        unsafe {
            sdk::IWorldObject::SetPosition(
                base_ptr_as!(world_object, ptr),
                value.x,
                value.y,
                value.z,
            )
        }
    }

    fn world_object_get_dimension(&self, ptr: altv_wasm_shared::BaseObjectPtr) -> i32 {
        unsafe { sdk::IWorldObject::GetDimension(base_ptr_as!(world_object, ptr)) }
    }

    fn world_object_set_dimension(&self, ptr: altv_wasm_shared::BaseObjectPtr, value: i32) {
        unsafe { sdk::IWorldObject::SetDimension(base_ptr_as!(world_object, ptr), value) }
    }

    fn create_local_vehicle(
        &self,
        model: u32,
        dimension: i32,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        rot_x: f32,
        rot_y: f32,
        rot_z: f32,
        use_streaming: bool,
        streaming_distance: u32,
    ) -> altv_wasm_shared::BaseObjectPtr {
        set_pending_base_object(true);
        let local_vehicle = unsafe {
            sdk::ICore::CreateLocalVehicle(
                model,
                dimension,
                pos_x,
                pos_y,
                pos_z,
                rot_x,
                rot_y,
                rot_z,
                use_streaming,
                streaming_distance,
                self.resource_ptr,
            )
        };
        set_pending_base_object(false);

        let ptr = unsafe { sdk::local_vehicle::to_base_object(local_vehicle) };
        ptr as altv_wasm_shared::BaseObjectPtr
    }

    fn vehicle_get_fuel_level(&self, ptr: altv_wasm_shared::BaseObjectPtr) -> f32 {
        let vehicle = unsafe { sdk::base_object::to_vehicle(ptr as _) };
        assert!(!vehicle.is_null());
        unsafe { sdk::IVehicle::GetFuelLevel(vehicle) }
    }

    fn vehicle_set_fuel_level(&self, ptr: altv_wasm_shared::BaseObjectPtr, value: f32) {
        let vehicle = unsafe { sdk::base_object::to_vehicle(ptr as _) };
        assert!(!vehicle.is_null());
        unsafe { sdk::IVehicle::SetFuelLevel(vehicle, value) }
    }

    fn entity_get_script_id(&self, ptr: altv_wasm_shared::BaseObjectPtr) -> u32 {
        let entity = unsafe { sdk::base_object::to_entity(ptr as _) };
        assert!(!entity.is_null());
        unsafe { sdk::IEntity::GetScriptID(entity) }
    }

    fn alloc_memory_buffer(&self, size: u16) -> MemoryBufferId {
        self.natives.memory_buffers.borrow_mut().alloc(size)
    }

    fn dealloc_memory_buffer(&self, id: MemoryBufferId) {
        self.natives.memory_buffers.borrow_mut().dealloc(id);
    }

    fn read_memory_buffer(&self, id: MemoryBufferId) -> Vec<u8> {
        self.natives.memory_buffers.borrow_mut().read(id)
    }
}
