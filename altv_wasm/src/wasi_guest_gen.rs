
    // AUTO-GENERATED
    // All manual changes will be overwritten

mod guest {
    mod __shared {
        pub type FatPtr = u64;
        pub type Size = u32;
        pub type Ptr = u32;
        pub fn from_fat_ptr(fat_ptr: FatPtr) -> (Ptr, Size) {
            let ptr = (fat_ptr >> 32) as Ptr;
            let size = fat_ptr as Size;
            (ptr, size)
        }
        pub fn to_fat_ptr(ptr: Ptr, size: Size) -> FatPtr {
            ((ptr as u64) << 32) | (size as u64)
        }
        const U64_SIZE: usize = std::mem::size_of::<u64>();
        pub const BYTES_TO_STORE_U64_32_TIMES: usize = 32 * U64_SIZE;
        type U64AsBytes = [u8; U64_SIZE];
        pub trait NumAsU64Arr: Copy {
            fn from_bytes(bytes: U64AsBytes) -> Self;
            fn into_bytes(self) -> U64AsBytes;
        }
        macro_rules! copy_to_full_arr {
            ($part:expr) => {
                { let mut bytes = [0u8; U64_SIZE]; bytes[.. $part .len()]
                .clone_from_slice(& $part); bytes }
            };
        }
        impl NumAsU64Arr for f32 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                f32::from_le_bytes(bytes[..4].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for f64 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                f64::from_le_bytes(bytes)
            }
            fn into_bytes(self) -> U64AsBytes {
                self.to_le_bytes()
            }
        }
        impl NumAsU64Arr for u8 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                u8::from_le_bytes(bytes[..1].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for u16 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                u16::from_le_bytes(bytes[..2].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for u32 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                u32::from_le_bytes(bytes[..4].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for u64 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                u64::from_le_bytes(bytes)
            }
            fn into_bytes(self) -> U64AsBytes {
                self.to_le_bytes()
            }
        }
        impl NumAsU64Arr for i8 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                i8::from_le_bytes(bytes[..1].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for i16 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                i16::from_le_bytes(bytes[..2].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for i32 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                i32::from_le_bytes(bytes[..4].try_into().unwrap())
            }
            fn into_bytes(self) -> U64AsBytes {
                copy_to_full_arr!(self.to_le_bytes())
            }
        }
        impl NumAsU64Arr for i64 {
            fn from_bytes(bytes: U64AsBytes) -> Self {
                i64::from_le_bytes(bytes)
            }
            fn into_bytes(self) -> U64AsBytes {
                self.to_le_bytes()
            }
        }
    }
    pub use __shared::{FatPtr, Ptr, Size};
    mod __internal {
        #[cfg(target_family = "wasm")]
        const _: () = assert!(
            std::mem::size_of:: < usize > () == std::mem::size_of:: < u32 > ()
        );
        #[no_mangle]
        pub fn __custom_free(fat_ptr: super::__shared::FatPtr) {
            let (ptr, size) = super::__shared::from_fat_ptr(fat_ptr);
            unsafe { std::alloc::dealloc(ptr as *mut u8, array_layout(size)) }
        }
        #[no_mangle]
        pub fn __custom_alloc(len: u32) -> super::__shared::Ptr {
            let ptr = unsafe { std::alloc::alloc(array_layout(len)) };
            if ptr.is_null() {
                panic!("Failed to allocate");
            }
            ptr as super::__shared::Ptr
        }
        #[no_mangle]
        pub fn __init_big_call(ptr: super::__shared::Ptr) {
            super::imports::BIG_CALL_PTR.set(ptr);
        }
        fn array_layout(len: u32) -> std::alloc::Layout {
            std::alloc::Layout::array::<u8>(len as usize).unwrap()
        }
        fn buffer_from_fat_ptr(fat_ptr: super::__shared::FatPtr) -> Vec<u8> {
            let (ptr, size) = super::__shared::from_fat_ptr(fat_ptr);
            unsafe { Vec::from_raw_parts(ptr as *mut u8, size as usize, size as usize) }
        }
        pub(super) fn send_to_host<T: ?Sized + serde::Serialize>(
            data: &T,
        ) -> super::__shared::FatPtr {
            let encoded = bincode::serialize(data).unwrap();
            let ptr = encoded.as_ptr();
            let size = encoded.len();
            std::mem::forget(encoded);
            super::__shared::to_fat_ptr(ptr as u32, size as u32)
        }
        pub(super) fn send_str_to_host(str: &str) -> super::__shared::FatPtr {
            let ptr = str.as_ptr();
            let size = str.len();
            super::__shared::to_fat_ptr(ptr as u32, size as u32)
        }
        pub(super) fn send_string_to_host(string: String) -> super::__shared::FatPtr {
            let ptr = string.as_ptr();
            let size = string.len();
            std::mem::forget(string);
            super::__shared::to_fat_ptr(ptr as u32, size as u32)
        }
        pub(super) fn read_from_host<T: serde::de::DeserializeOwned>(
            fat_ptr: super::__shared::FatPtr,
        ) -> T {
            let buffer = buffer_from_fat_ptr(fat_ptr);
            bincode::deserialize(&buffer).unwrap()
        }
        pub(super) fn read_string_from_host(fat_ptr: super::__shared::FatPtr) -> String {
            let buffer = buffer_from_fat_ptr(fat_ptr);
            String::from_utf8(buffer).unwrap()
        }
    }
    pub mod imports {
        thread_local! {
            pub (super) static BIG_CALL_PTR : std::cell::Cell < super::__shared::Ptr > =
            std::cell::Cell::new(0);
        }
        pub fn log(message: &String) {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(log)]
                fn __custom_imports_log(message: super::__shared::FatPtr);
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let message = super::__internal::send_str_to_host(message);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_log(message) };
            }
        }
        pub fn log_error(message: &String) {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(log_error)]
                fn __custom_imports_log_error(message: super::__shared::FatPtr);
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let message = super::__internal::send_str_to_host(message);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_log_error(message) };
            }
        }
        pub fn destroy_base_object(ptr: altv_wasm_shared::BaseObjectPtr) {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(destroy_base_object)]
                fn __custom_imports_destroy_base_object(ptr: u64);
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let ptr = ptr as u64;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_destroy_base_object(ptr) };
            }
        }
        pub fn base_object_get_id(ptr: altv_wasm_shared::BaseObjectPtr) -> u32 {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(base_object_get_id)]
                fn __custom_imports_base_object_get_id(ptr: u64) -> u32;
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let ptr = ptr as u64;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_base_object_get_id(ptr) };
                call_return as u32
            }
        }
        pub fn base_object_get_remote_id(ptr: altv_wasm_shared::BaseObjectPtr) -> u32 {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(base_object_get_remote_id)]
                fn __custom_imports_base_object_get_remote_id(ptr: u64) -> u32;
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let ptr = ptr as u64;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe {
                    __custom_imports_base_object_get_remote_id(ptr)
                };
                call_return as u32
            }
        }
        pub fn world_object_get_pos(
            ptr: altv_wasm_shared::BaseObjectPtr,
        ) -> shared::Vector3 {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(world_object_get_pos)]
                fn __custom_imports_world_object_get_pos(
                    ptr: u64,
                ) -> super::__shared::FatPtr;
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let ptr = ptr as u64;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_world_object_get_pos(ptr) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn world_object_set_pos(
            ptr: altv_wasm_shared::BaseObjectPtr,
            value: shared::Vector3,
        ) {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(world_object_set_pos)]
                fn __custom_imports_world_object_set_pos(
                    ptr: u64,
                    value: super::__shared::FatPtr,
                );
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let ptr = ptr as u64;
                let value = super::__internal::send_to_host(&value);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe {
                    __custom_imports_world_object_set_pos(ptr, value)
                };
            }
        }
        pub fn world_object_get_dimension(ptr: altv_wasm_shared::BaseObjectPtr) -> i32 {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(world_object_get_dimension)]
                fn __custom_imports_world_object_get_dimension(ptr: u64) -> i32;
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let ptr = ptr as u64;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe {
                    __custom_imports_world_object_get_dimension(ptr)
                };
                call_return as i32
            }
        }
        pub fn world_object_set_dimension(
            ptr: altv_wasm_shared::BaseObjectPtr,
            value: i32,
        ) {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(world_object_set_dimension)]
                fn __custom_imports_world_object_set_dimension(ptr: u64, value: i32);
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let ptr = ptr as u64;
                let value = value as i32;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe {
                    __custom_imports_world_object_set_dimension(ptr, value)
                };
            }
        }
        pub fn create_local_vehicle(
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
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(create_local_vehicle)]
                fn __custom_imports_create_local_vehicle(
                    model: u32,
                    dimension: i32,
                    pos_x: f32,
                    pos_y: f32,
                    pos_z: f32,
                    rot_x: f32,
                    rot_y: f32,
                    rot_z: f32,
                    use_streaming: i32,
                    streaming_distance: u32,
                ) -> u64;
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let model = model as u32;
                let dimension = dimension as i32;
                let pos_x = pos_x as f32;
                let pos_y = pos_y as f32;
                let pos_z = pos_z as f32;
                let rot_x = rot_x as f32;
                let rot_y = rot_y as f32;
                let rot_z = rot_z as f32;
                let use_streaming = use_streaming as i32;
                let streaming_distance = streaming_distance as u32;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe {
                    __custom_imports_create_local_vehicle(
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
                    )
                };
                call_return as altv_wasm_shared::BaseObjectPtr
            }
        }
        pub fn vehicle_set_fuel_level(ptr: altv_wasm_shared::BaseObjectPtr, value: f32) {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(vehicle_set_fuel_level)]
                fn __custom_imports_vehicle_set_fuel_level(ptr: u64, value: f32);
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let ptr = ptr as u64;
                let value = value as f32;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe {
                    __custom_imports_vehicle_set_fuel_level(ptr, value)
                };
            }
        }
        pub fn vehicle_get_fuel_level(ptr: altv_wasm_shared::BaseObjectPtr) -> f32 {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(vehicle_get_fuel_level)]
                fn __custom_imports_vehicle_get_fuel_level(ptr: u64) -> f32;
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let ptr = ptr as u64;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe {
                    __custom_imports_vehicle_get_fuel_level(ptr)
                };
                call_return as f32
            }
        }
        pub fn alloc_memory_buffer(size: u16) -> shared::MemoryBufferId {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(alloc_memory_buffer)]
                fn __custom_imports_alloc_memory_buffer(size: u32) -> u32;
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let size = size as u32;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_alloc_memory_buffer(size) };
                call_return as shared::MemoryBufferId
            }
        }
        pub fn dealloc_memory_buffer(id: shared::MemoryBufferId) {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(dealloc_memory_buffer)]
                fn __custom_imports_dealloc_memory_buffer(id: u32);
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let id = id as u32;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_dealloc_memory_buffer(id) };
            }
        }
        pub fn read_memory_buffer(id: shared::MemoryBufferId) -> Vec<u8> {
            #[link(wasm_import_module = "__custom_imports")]
            extern "C" {
                #[link_name = stringify!(read_memory_buffer)]
                fn __custom_imports_read_memory_buffer(
                    id: u32,
                ) -> super::__shared::FatPtr;
            }
            #[allow(clippy::unnecessary_cast)]
            {
                let id = id as u32;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_read_memory_buffer(id) };
                super::__internal::read_from_host(call_return)
            }
        }
        #[link(wasm_import_module = "__custom_imports")]
        extern "C" {
            #[link_name = "native"]
            fn __custom_imports_native(func_index: u32) -> u64;
        }
        pub fn native_get_dlc_weapon_data(
            dlc_weapon_index_: i32,
            out_data_: shared::MemoryBufferId,
        ) -> altv_wasm_shared::natives_result::ResultOfGetDlcWeaponData {
            #[allow(clippy::unnecessary_cast)]
            {
                let mut big_call_args = unsafe {
                    let mut args = Vec::from_raw_parts(
                        BIG_CALL_PTR.get() as *mut u8,
                        super::__shared::BYTES_TO_STORE_U64_32_TIMES,
                        super::__shared::BYTES_TO_STORE_U64_32_TIMES,
                    );
                    args.set_len(0);
                    args
                };
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            dlc_weapon_index_ as i32,
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(out_data_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(0u32) };
                super::__internal::read_from_host(call_return)
            }
        }
    }
    const _: &str = include_str!(
        r#"C:\\dev\\rust\\altv-rust\\altv_wasm\\../wasm.interface"#
    );
    const _: &str = include_str!(
        r#"C:\\dev\\rust\\altv-rust\\altv_wasm\\../wasm_natives.interface"#
    );
    pub mod exports {
        pub trait Exports {
            fn on_tick();
            fn on_base_object_create(
                ptr: altv_wasm_shared::BaseObjectPtr,
                ty: altv_wasm_shared::BaseObjectTypeRaw,
            );
            fn on_base_object_destroy(
                ptr: altv_wasm_shared::BaseObjectPtr,
                ty: altv_wasm_shared::BaseObjectTypeRaw,
            );
        }
        pub struct ExportsImpl;
        #[no_mangle]
        extern "C" fn __custom_exports_on_tick() {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = <ExportsImpl as Exports>::on_tick();
            }
        }
        #[no_mangle]
        extern "C" fn __custom_exports_on_base_object_create(ptr: u64, ty: u32) {
            #[allow(clippy::unnecessary_cast)]
            {
                let ptr = ptr as altv_wasm_shared::BaseObjectPtr;
                let ty = ty as altv_wasm_shared::BaseObjectTypeRaw;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = <ExportsImpl as Exports>::on_base_object_create(
                    ptr,
                    ty,
                );
            }
        }
        #[no_mangle]
        extern "C" fn __custom_exports_on_base_object_destroy(ptr: u64, ty: u32) {
            #[allow(clippy::unnecessary_cast)]
            {
                let ptr = ptr as altv_wasm_shared::BaseObjectPtr;
                let ty = ty as altv_wasm_shared::BaseObjectTypeRaw;
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = <ExportsImpl as Exports>::on_base_object_destroy(
                    ptr,
                    ty,
                );
            }
        }
    }
    const _: &str = include_str!(
        r#"C:\\dev\\rust\\altv-rust\\altv_wasm\\../wasm.interface"#
    );
}
pub use guest::*;
