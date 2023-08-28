use wasmtime::*;
use wasmtime_wasi::{WasiCtxBuilder, WasiCtx};
use altv_sdk::ffi as sdk;

use crate::resource_manager::set_pending_base_object;

wasm_codegen::host!("../wasm.interface");
pub type Exports = host::exports::Exports<State>;

impl std::fmt::Debug for Exports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Exports {{}}")
    }
}

pub struct State {
    wasi: WasiCtx,

    // TODO: some safe wrapper over this unsafe shit
    resource_ptr: *mut sdk::shared::AltResource,
}

impl host::imports::Imports for State {
    fn log_error(&self, message: String) {
        unsafe { altv_sdk::helpers::log_error_with_resource(&message, self.resource_ptr) }
    }

    fn log(&self, message: String) {
        unsafe { altv_sdk::helpers::log_with_resource(&message, self.resource_ptr) }
    }

    fn destroy_base_object(&self, ptr: altv_wasm_shared::BaseObjectPtr) {
        set_pending_base_object(true);
        unsafe { sdk::ICore::DestroyBaseObject(ptr as _) }
        set_pending_base_object(false);
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
        assert!(!ptr.is_null());
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

    fn base_object_get_id(&self, ptr: altv_wasm_shared::BaseObjectPtr) -> u32 {
        unsafe { sdk::IBaseObject::GetID(ptr as _) }
    }

    fn base_object_get_remote_id(&self, ptr: altv_wasm_shared::BaseObjectPtr) -> u32 {
        unsafe { sdk::IBaseObject::GetRemoteID(ptr as _) }
    }
}

pub(crate) fn start(
    wasm_bytes: &[u8],
    resource_ptr: *mut sdk::shared::AltResource,
) -> wasmtime::Result<Exports> {
    std::env::set_var("WASMTIME_BACKTRACE_DETAILS", "1");

    let engine = Engine::default();
    let mut linker = Linker::<State>::new(&engine);

    let module = Module::from_binary(&engine, wasm_bytes)?;

    let wasi = WasiCtxBuilder::new().build();
    let mut store = Store::new(&engine, State { wasi, resource_ptr });

    wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi)?;
    host::imports::add_to_linker(&mut linker);

    let instance = linker.instantiate(&mut store, &module)?;

    let pre_main = instance.get_typed_func::<(), ()>(&mut store, "pre_main")?;
    pre_main.call(&mut store, ())?;

    let main = instance.get_typed_func::<(), ()>(&mut store, "main")?;
    main.call(&mut store, ())?;

    Ok(Exports::new(store, instance))
}
