use autocxx::prelude::*;
use wasmtime_wasi::WasiCtx;
use altv_sdk::ffi as sdk;
use crate::{resource_manager::set_pending_base_object, helpers::read_cpp_vector3};

wasm_codegen::host!("../wasm.interface");
pub use host::imports;

use self::imports::Imports;

pub type Exports = host::exports::Exports<State>;

impl std::fmt::Debug for Exports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Exports {{}}")
    }
}

pub struct State {
    pub wasi: WasiCtx,

    // TODO: some safe wrapper over this unsafe shit
    pub resource_ptr: *mut sdk::shared::AltResource,
    pub big_call_ptr: host::Ptr,

    memory: Option<wasmtime::Memory>,
    free: Option<host::FreeFunc>,
    alloc: Option<host::AllocFunc>,
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

impl host::imports::Imports for State {
    fn get_big_call_ptr(&self) -> u32 {
        self.big_call_ptr
    }

    fn get_memory(&self) -> Option<wasmtime::Memory> {
        self.memory
    }

    fn get_free(&self) -> Option<host::FreeFunc> {
        self.free
    }

    fn get_alloc(&self) -> Option<host::AllocFunc> {
        self.alloc
    }

    fn set_memory(&mut self, memory: wasmtime::Memory) {
        self.memory.replace(memory);
    }

    fn set_free(&mut self, value: host::FreeFunc) {
        self.free.replace(value);
    }

    fn set_alloc(&mut self, alloc: host::AllocFunc) {
        self.alloc.replace(alloc);
    }

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

    fn base_object_get_id(&self, ptr: altv_wasm_shared::BaseObjectPtr) -> u32 {
        unsafe { sdk::IBaseObject::GetID(ptr as _) }
    }

    fn base_object_get_remote_id(&self, ptr: altv_wasm_shared::BaseObjectPtr) -> u32 {
        unsafe { sdk::IBaseObject::GetRemoteID(ptr as _) }
    }

    fn world_object_get_pos(
        &self,
        ptr: altv_wasm_shared::BaseObjectPtr,
    ) -> altv_wasm_shared::Vector3 {
        read_cpp_vector3(
            unsafe { sdk::IWorldObject::GetPosition(base_ptr_as!(world_object, ptr)) }
                .within_unique_ptr(),
        )
    }

    fn world_object_set_pos(
        &self,
        ptr: altv_wasm_shared::BaseObjectPtr,
        // TODO: maybe use three parameters instead for better performance?
        value: altv_wasm_shared::Vector3,
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

    fn natives_do_screen_fade_out(&self, duration: i32) {
        unsafe {
            let mut success = Default::default();
            sdk::natives::do_screen_fade_out(&mut success, duration);
            logger::debug!("do_screen_fade_out success: {success}");
        }
    }

    fn natives_do_screen_fade_in(&self, duration: i32) {
        unsafe {
            let mut success = Default::default();
            sdk::natives::do_screen_fade_in(&mut success, duration);
            logger::debug!("do_screen_fade_in success: {success}");
        }
    }

    fn natives_play_sound(
        &self,
        soundId: i32,
        _audioName: String,
        _audioRef: String,
        _p3: u8,
        _p4: i32,
        _p5: u8,
    ) {
        unsafe {
            let mut success = Default::default();
            sdk::natives::play_sound(&mut success, soundId, _audioName, _audioRef, _p3, _p4, _p5);
            // logger::debug!("play_sound success: {success}");
        }
    }

    fn natives_play_sound_from_coord(
        &self,
        soundId: i32,
        _audioName: String,
        x: f32,
        y: f32,
        z: f32,
        _audioRef: String,
        _isNetwork: u8,
        _range: i32,
        _p8: u8,
    ) {
        unsafe {
            let mut success = Default::default();
            sdk::natives::play_sound_from_coord(
                &mut success,
                soundId,
                _audioName,
                x,
                y,
                z,
                _audioRef,
                _isNetwork,
                _range,
                _p8,
            );
            // logger::debug!("play_sound_from_coord success: {success}");
        }
    }

    fn natives_get_sound_id(&self) -> i32 {
        unsafe {
            let mut success = Default::default();
            let result = sdk::natives::get_sound_id(&mut success);
            // logger::debug!("get_sound_id success: {success}");
            result
        }
    }

    fn natives_release_sound_id(&self, sound_id: i32) {
        unsafe {
            let mut success = Default::default();
            sdk::natives::release_sound_id(&mut success, sound_id);
            // logger::debug!("release_sound_id success: {success}");
        }
    }

    fn natives_stop_sound(&self, sound_id: i32) {
        unsafe {
            let mut success = Default::default();
            sdk::natives::stop_sound(&mut success, sound_id);
            // logger::debug!("stop_sound success: {success}");
        }
    }

    fn natives_draw_marker(
        &self,
        type_: i32,
        x: f32,
        y: f32,
        z: f32,
        dirX_: f32,
        dirY_: f32,
        dirZ_: f32,
        rotX_: f32,
        rotY_: f32,
        rotZ_: f32,
        scaleX_: f32,
        scaleY_: f32,
        scaleZ_: f32,
        red_: i32,
        green_: i32,
        blue_: i32,
        alpha_: i32,
        bobUpAndDown_: bool,
        faceCamera_: bool,
        p19_: i32,
        rotate_: bool,
        textureDict: String,
        textureName: String,
        drawOnEnts_: bool,
    ) {
        logger::debug!("draw_marker {x}, {y}, {z}");
        let success = unsafe {
            let texture_dict = sdk::natives::create_c_string_ptr(textureDict).within_unique_ptr();
            let texture_name = sdk::natives::create_c_string_ptr(textureName).within_unique_ptr();
            sdk::natives::draw_marker(
                type_,
                x,
                y,
                z,
                dirX_,
                dirY_,
                dirZ_,
                rotX_,
                rotY_,
                rotZ_,
                scaleX_,
                scaleY_,
                scaleZ_,
                red_,
                green_,
                blue_,
                alpha_,
                bobUpAndDown_,
                faceCamera_,
                p19_,
                rotate_,
                texture_dict.as_ref().unwrap(),
                texture_name.as_ref().unwrap(),
                drawOnEnts_,
            )
        };
        logger::debug!("draw_marker success: {success}");
    }

    fn request_streamed_texture_dict(&self, dict: String) {
        logger::debug!("texture dict: {dict:?}");

        let success = unsafe {
            let dict = sdk::natives::create_c_string_ptr(dict).within_unique_ptr();
            sdk::natives::request_streamed_texture_dict(dict.as_ref().unwrap(), false)
        };

        logger::debug!("request_streamed_texture_dict success: {success}");
    }
}
