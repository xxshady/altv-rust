
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
        #[link(wasm_import_module = "__custom_imports")]
        extern "C" {
            #[link_name = "native"]
            fn __custom_imports_native(func_index: u32) -> u64;
        }
        pub fn native_app_get_float(
            property_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_app_get_float {
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
                            super::__internal::send_to_host(&property_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(0u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_set_float(
            property_: Option<&String>,
            value_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_app_set_float {
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
                            super::__internal::send_to_host(&property_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(value_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(1u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_set_block(
            blockName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_app_set_block {
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
                            super::__internal::send_to_host(&blockName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(2u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_set_string(
            property_: Option<&String>,
            value_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_app_set_string {
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
                            super::__internal::send_to_host(&property_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&value_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(3u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_delete_app_data(
            appName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_app_delete_app_data {
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
                            super::__internal::send_to_host(&appName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(4u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_clear_block() -> altv_wasm_shared::natives_result::ResultOf_app_clear_block {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(5u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_set_int(
            property_: Option<&String>,
            value_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_app_set_int {
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
                            super::__internal::send_to_host(&property_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(value_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(6u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_has_linked_social_club_account() -> altv_wasm_shared::natives_result::ResultOf_app_has_linked_social_club_account {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(7u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_get_string(
            property_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_app_get_string {
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
                            super::__internal::send_to_host(&property_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(8u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_data_valid() -> altv_wasm_shared::natives_result::ResultOf_app_data_valid {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(9u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_save_data() -> altv_wasm_shared::natives_result::ResultOf_app_save_data {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(10u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_get_deleted_file_status() -> altv_wasm_shared::natives_result::ResultOf_app_get_deleted_file_status {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(11u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_has_synced_data(
            appName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_app_has_synced_data {
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
                            super::__internal::send_to_host(&appName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(12u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_set_app(
            appName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_app_set_app {
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
                            super::__internal::send_to_host(&appName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(13u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_get_int(
            property_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_app_get_int {
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
                            super::__internal::send_to_host(&property_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(14u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_close_app() -> altv_wasm_shared::natives_result::ResultOf_app_close_app {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(15u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_app_close_block() -> altv_wasm_shared::natives_result::ResultOf_app_close_block {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(16u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_start_audio_scene(
            scene_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_start_audio_scene {
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
                            super::__internal::send_to_host(&scene_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(17u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_unrequest_tennis_banks() -> altv_wasm_shared::natives_result::ResultOf_unrequest_tennis_banks {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(18u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_vehicle_audio_body_damage_factor(
            vehicle_: u32,
            intensity_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_vehicle_audio_body_damage_factor {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(intensity_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(19u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_ambient_zone_enabled(
            ambientZone_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_ambient_zone_enabled {
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
                            super::__internal::send_to_host(&ambientZone_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(20u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_vehicle_default_horn(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_vehicle_default_horn {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(21u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_prepare_synchronized_audio_event_for_scene(
            sceneID_: i32,
            audioEvent_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_prepare_synchronized_audio_event_for_scene {
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
                        &super::__shared::NumAsU64Arr::into_bytes(sceneID_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&audioEvent_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(22u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_radio_position_audio_mute(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_radio_position_audio_mute {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(23u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_unlock_radio_station_track_list(
            radioStation_: Option<&String>,
            trackListName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_unlock_radio_station_track_list {
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
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&trackListName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(24u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_can_vehicle_receive_cb_radio(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_can_vehicle_receive_cb_radio {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(25u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_vehicle_horn_sound_index(
            vehicle_: u32,
            value_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_vehicle_horn_sound_index {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(value_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(26u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_start_alarm(
            alarmName_: Option<&String>,
            p2_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_start_alarm {
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
                            super::__internal::send_to_host(&alarmName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(27u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_portal_settings_override(
            p0_: Option<&String>,
            p1_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_portal_settings_override {
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
                            super::__internal::send_to_host(&p0_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p1_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(28u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_ped_in_current_conversation(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_ped_in_current_conversation {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(29u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_radio_faded_out() -> altv_wasm_shared::natives_result::ResultOf_is_radio_faded_out {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(30u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_force_ped_panic_walla() -> altv_wasm_shared::natives_result::ResultOf_force_ped_panic_walla {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(31u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ped_footsteps_events_enabled(
            ped_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ped_footsteps_events_enabled {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(32u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_script_update_door_audio(
            doorHash_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_script_update_door_audio {
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
                        &super::__shared::NumAsU64Arr::into_bytes(doorHash_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(33u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_conversation_audio_controlled_by_anim(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_conversation_audio_controlled_by_anim {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(34u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ped_voice_group_from_race_to_pvg(
            ped_: u32,
            voiceGroupHash_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ped_voice_group_from_race_to_pvg {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(voiceGroupHash_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(35u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_vehicle_radio_on(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_vehicle_radio_on {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(36u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_release_mission_audio_bank() -> altv_wasm_shared::natives_result::ResultOf_release_mission_audio_bank {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(37u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_does_player_veh_have_radio() -> altv_wasm_shared::natives_result::ResultOf_does_player_veh_have_radio {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(38u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_mobile_radio_enabled_during_gameplay(
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_mobile_radio_enabled_during_gameplay {
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
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(39u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_unhint_named_script_audio_bank(
            audioBank_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_unhint_named_script_audio_bank {
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
                            super::__internal::send_to_host(&audioBank_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(40u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_clear_ambient_zone_list_state(
            ambientZone_: Option<&String>,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_clear_ambient_zone_list_state {
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
                            super::__internal::send_to_host(&ambientZone_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(41u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_audio_special_effect_mode(
            mode_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_audio_special_effect_mode {
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
                        &super::__shared::NumAsU64Arr::into_bytes(mode_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(42u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_override_trevor_rage(
            voiceEffect_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_override_trevor_rage {
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
                            super::__internal::send_to_host(&voiceEffect_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(43u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ped_walla_density(
            p0_: f32,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ped_walla_density {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(44u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_add_entity_to_audio_mix_group(
            entity_: u32,
            groupName_: Option<&String>,
            p2_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_add_entity_to_audio_mix_group {
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
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&groupName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(45u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_global_radio_signal_level(
            p0_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_global_radio_signal_level {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(46u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_clear_custom_radio_track_list(
            radioStation_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_clear_custom_radio_track_list {
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
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(47u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_scripted_conversation_ongoing() -> altv_wasm_shared::natives_result::ResultOf_is_scripted_conversation_ongoing {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(48u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_remove_entity_from_audio_mix_group(
            entity_: u32,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_remove_entity_from_audio_mix_group {
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
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(49u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_mission_complete_playing() -> altv_wasm_shared::natives_result::ResultOf_is_mission_complete_playing {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(50u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_unhint_ambient_audio_bank() -> altv_wasm_shared::natives_result::ResultOf_unhint_ambient_audio_bank {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(51u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_user_radio_control_enabled(
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_user_radio_control_enabled {
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
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(52u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ped_race_and_voice_group(
            ped_: u32,
            p1_: i32,
            voiceGroup_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ped_race_and_voice_group {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(voiceGroup_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(53u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_blip_siren(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_blip_siren {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(54u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_veh_radio_station(
            vehicle_: u32,
            radioStation_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_veh_radio_station {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(55u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_enable_vehicle_fanbelt_damage(
            vehicle_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_enable_vehicle_fanbelt_damage {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(56u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ambient_zone_state_persistent(
            ambientZone_: Option<&String>,
            p1_: bool,
            p2_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ambient_zone_state_persistent {
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
                            super::__internal::send_to_host(&ambientZone_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(57u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_prepare_music_event(
            eventName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_prepare_music_event {
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
                            super::__internal::send_to_host(&eventName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(58u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_ped_ringtone_playing(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_ped_ringtone_playing {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(59u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_load_stream(
            streamName_: Option<&String>,
            soundSet_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_load_stream {
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
                            super::__internal::send_to_host(&streamName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&soundSet_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(60u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_siren_with_no_driver(
            vehicle_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_siren_with_no_driver {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(61u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_stream_from_position(
            x_: f32,
            y_: f32,
            z_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_stream_from_position {
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
                        &super::__shared::NumAsU64Arr::into_bytes(x_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(62u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_clear_ambient_zone_state(
            zoneName_: Option<&String>,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_clear_ambient_zone_state {
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
                            super::__internal::send_to_host(&zoneName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(63u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_alarm_playing(
            alarmName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_alarm_playing {
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
                            super::__internal::send_to_host(&alarmName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(64u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_start_preloaded_conversation() -> altv_wasm_shared::natives_result::ResultOf_start_preloaded_conversation {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(65u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_start_script_phone_conversation(
            p0_: bool,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_start_script_phone_conversation {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(66u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ped_cloth_events_enabled(
            ped_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ped_cloth_events_enabled {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(67u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_unblock_speech_context_group(
            p0_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_unblock_speech_context_group {
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
                            super::__internal::send_to_host(&p0_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(68u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_radio_station_favourited(
            radioStation_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_radio_station_favourited {
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
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(69u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_enable_vehicle_exhaust_pops(
            vehicle_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_enable_vehicle_exhaust_pops {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(70u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_radio_frontend_fade_time(
            fadeTime_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_radio_frontend_fade_time {
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
                        &super::__shared::NumAsU64Arr::into_bytes(fadeTime_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(71u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_radio_track_with_start_offset(
            radioStationName_: Option<&String>,
            mixName_: Option<&String>,
            p2_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_radio_track_with_start_offset {
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
                            super::__internal::send_to_host(&radioStationName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&mixName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(72u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_audio_is_scripted_music_playing() -> altv_wasm_shared::natives_result::ResultOf_audio_is_scripted_music_playing {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(73u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_network_id_from_sound_id(
            soundId_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_network_id_from_sound_id {
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
                        &super::__shared::NumAsU64Arr::into_bytes(soundId_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(74u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_all_alarms(
            stop_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_all_alarms {
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
                        &super::__shared::NumAsU64Arr::into_bytes(stop_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(75u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_request_script_audio_bank(
            audioBank_: Option<&String>,
            p1_: bool,
            p2_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_request_script_audio_bank {
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
                            super::__internal::send_to_host(&audioBank_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(76u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_variable_on_stream(
            unkVariable_: Option<&String>,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_variable_on_stream {
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
                            super::__internal::send_to_host(&unkVariable_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(77u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_any_positional_speech_playing() -> altv_wasm_shared::natives_result::ResultOf_is_any_positional_speech_playing {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(78u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_position_for_null_conv_ped(
            p0_: i32,
            p1_: f32,
            p2_: f32,
            p3_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_position_for_null_conv_ped {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(79u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_freeze_radio_station(
            radioStation_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_freeze_radio_station {
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
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(80u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_current_track_sound_name(
            radioStationName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_current_track_sound_name {
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
                            super::__internal::send_to_host(&radioStationName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(81u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_ped_ambient_speech_with_voice_native(
            ped_: u32,
            speechName_: Option<&String>,
            voiceName_: Option<&String>,
            speechParam_: Option<&String>,
            p4_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_ped_ambient_speech_with_voice_native {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&speechName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&voiceName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&speechParam_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(82u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_release_sound_id(
            soundId_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_release_sound_id {
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
                        &super::__shared::NumAsU64Arr::into_bytes(soundId_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(83u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_aggressive_horns(
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_aggressive_horns {
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
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(84u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_static_emitter_enabled(
            emitterName_: Option<&String>,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_static_emitter_enabled {
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
                            super::__internal::send_to_host(&emitterName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(85u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_music_vol_slider() -> altv_wasm_shared::natives_result::ResultOf_get_music_vol_slider {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(86u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_vehicle_door_open_sound(
            vehicle_: u32,
            doorId_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_vehicle_door_open_sound {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(doorId_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(87u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_preload_script_conversation(
            p0_: bool,
            p1_: bool,
            p2_: bool,
            p3_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_preload_script_conversation {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(88u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cutscene_audio_override(
            name_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cutscene_audio_override {
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
                            super::__internal::send_to_host(&name_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(89u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_vehicle_radio_enabled(
            vehicle_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_vehicle_radio_enabled {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(90u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gps_active(
            active_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gps_active {
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
                        &super::__shared::NumAsU64Arr::into_bytes(active_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(91u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_override_veh_horn(
            vehicle_: u32,
            override_: bool,
            hornHash_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_override_veh_horn {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(override_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(hornHash_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(92u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_veh_has_normal_radio(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_veh_has_normal_radio {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(93u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_current_track_play_time(
            radioStationName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_current_track_play_time {
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
                            super::__internal::send_to_host(&radioStationName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(94u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_hint_mission_audio_bank(
            audioBank_: Option<&String>,
            p1_: bool,
            p2_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_hint_mission_audio_bank {
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
                            super::__internal::send_to_host(&audioBank_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(95u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ped_voice_full(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ped_voice_full {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(96u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_sound_id() -> altv_wasm_shared::natives_result::ResultOf_get_sound_id {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(97u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_siren_can_be_controlled_by_audio(
            vehicle_: u32,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_siren_can_be_controlled_by_audio {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(98u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_lock_radio_station(
            radioStationName_: Option<&String>,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_lock_radio_station {
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
                            super::__internal::send_to_host(&radioStationName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(99u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_update_unlockable_dj_radio_tracks(
            enableMixes_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_update_unlockable_dj_radio_tracks {
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
                        &super::__shared::NumAsU64Arr::into_bytes(enableMixes_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(100u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_current_scripted_conversation_line() -> altv_wasm_shared::natives_result::ResultOf_get_current_scripted_conversation_line {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(101u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_does_context_exist_for_this_ped(
            ped_: u32,
            speechName_: Option<&String>,
            p2_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_does_context_exist_for_this_ped {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&speechName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(102u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_vehicle_boost_active(
            vehicle_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_vehicle_boost_active {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(103u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_request_tennis_banks(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_request_tennis_banks {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(104u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_radio_station_as_favourite(
            radioStation_: Option<&String>,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_radio_station_as_favourite {
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
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(105u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_force_music_track_list(
            radioStation_: Option<&String>,
            trackListName_: Option<&String>,
            milliseconds_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_force_music_track_list {
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
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&trackListName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(milliseconds_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(106u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_custom_radio_track_list(
            radioStation_: Option<&String>,
            trackListName_: Option<&String>,
            p2_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_custom_radio_track_list {
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
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&trackListName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(107u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_stream_play_time() -> altv_wasm_shared::natives_result::ResultOf_get_stream_play_time {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(108u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_force_use_audio_game_object(
            vehicle_: u32,
            audioName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_force_use_audio_game_object {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&audioName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(109u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_audible_music_track_text_id() -> altv_wasm_shared::natives_result::ResultOf_get_audible_music_track_text_id {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(110u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_has_loaded_mp_data_set() -> altv_wasm_shared::natives_result::ResultOf_has_loaded_mp_data_set {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(111u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_distant_cop_car_sirens(
            value_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_distant_cop_car_sirens {
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
                        &super::__shared::NumAsU64Arr::into_bytes(value_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(112u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_next_radio_track(
            radioName_: Option<&String>,
            radioTrack_: Option<&String>,
            p2_: Option<&String>,
            p3_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_next_radio_track {
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
                            super::__internal::send_to_host(&radioName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&radioTrack_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p2_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p3_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(113u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_vehicle_conversations_persist(
            p0_: bool,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_vehicle_conversations_persist {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(114u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_stream_frontend() -> altv_wasm_shared::natives_result::ResultOf_play_stream_frontend {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(115u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_load_stream_with_start_offset(
            streamName_: Option<&String>,
            startOffset_: i32,
            soundSet_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_load_stream_with_start_offset {
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
                            super::__internal::send_to_host(&streamName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(startOffset_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&soundSet_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(116u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_vehicle_audio_engine_damage_factor(
            vehicle_: u32,
            damageFactor_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_vehicle_audio_engine_damage_factor {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(damageFactor_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(117u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_cancel_music_event(
            eventName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_cancel_music_event {
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
                            super::__internal::send_to_host(&eventName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(118u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_has_loaded_sp_data_set() -> altv_wasm_shared::natives_result::ResultOf_has_loaded_sp_data_set {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(119u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_sound_from_entity_hash(
            soundId_: i32,
            model_: u32,
            entity_: u32,
            soundSetHash_: u32,
            p4_: i32,
            p5_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_sound_from_entity_hash {
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
                        &super::__shared::NumAsU64Arr::into_bytes(soundId_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(model_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(soundSetHash_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p5_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(120u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_refresh_closest_ocean_shoreline() -> altv_wasm_shared::natives_result::ResultOf_refresh_closest_ocean_shoreline {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(121u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_vehicle_audibly_damaged(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_vehicle_audibly_damaged {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(122u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_ambient_voice_name_hash(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_ambient_voice_name_hash {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(123u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_player_veh_radio_enable() -> altv_wasm_shared::natives_result::ResultOf_is_player_veh_radio_enable {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(124u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_preload_script_phone_conversation(
            p0_: bool,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_preload_script_phone_conversation {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(125u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_conversation_audio_placeholder(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_conversation_audio_placeholder {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(126u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_vehicle_door_close_sound(
            vehicle_: u32,
            doorId_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_vehicle_door_close_sound {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(doorId_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(127u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_link_static_emitter_to_entity(
            emitterName_: Option<&String>,
            entity_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_link_static_emitter_to_entity {
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
                            super::__internal::send_to_host(&emitterName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(128u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_release_ambient_audio_bank() -> altv_wasm_shared::natives_result::ResultOf_release_ambient_audio_bank {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(129u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_trigger_siren_audio(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_trigger_siren_audio {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(130u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_mission_news_story_unlocked(
            newsStory_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_mission_news_story_unlocked {
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
                        &super::__shared::NumAsU64Arr::into_bytes(newsStory_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(131u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_sound_frontend(
            soundId_: i32,
            audioName_: Option<&String>,
            audioRef_: Option<&String>,
            p3_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_sound_frontend {
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
                        &super::__shared::NumAsU64Arr::into_bytes(soundId_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&audioName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&audioRef_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(132u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_start_script_conversation(
            p0_: bool,
            p1_: bool,
            p2_: bool,
            p3_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_start_script_conversation {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(133u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_ped_ringtone(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_ped_ringtone {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(134u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ambient_voice_name(
            ped_: u32,
            name_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ambient_voice_name {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&name_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(135u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_game_in_control_of_music() -> altv_wasm_shared::natives_result::ResultOf_is_game_in_control_of_music {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(136u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_skip_radio_forward() -> altv_wasm_shared::natives_result::ResultOf_skip_radio_forward {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(137u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_mission_complete_ready_for_ui() -> altv_wasm_shared::natives_result::ResultOf_is_mission_complete_ready_for_ui {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(138u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_player_vehicle_alarm_audio_active(
            vehicle_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_player_vehicle_alarm_audio_active {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(139u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_trigger_music_event(
            eventName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_trigger_music_event {
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
                            super::__internal::send_to_host(&eventName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(140u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_script_overrides_wind_elevation(
            p0_: bool,
            p1_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_script_overrides_wind_elevation {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(141u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_any_speech_playing(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_any_speech_playing {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(142u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_variable_on_under_water_stream(
            unkVariableName_: Option<&String>,
            value_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_variable_on_under_water_stream {
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
                            super::__internal::send_to_host(&unkVariableName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(value_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(143u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_request_mission_audio_bank(
            audioBank_: Option<&String>,
            p1_: bool,
            p2_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_request_mission_audio_bank {
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
                            super::__internal::send_to_host(&audioBank_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(144u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_mobile_phone_call_ongoing() -> altv_wasm_shared::natives_result::ResultOf_is_mobile_phone_call_ongoing {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(145u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_sound_id_from_network_id(
            netId_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_sound_id_from_network_id {
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
                        &super::__shared::NumAsU64Arr::into_bytes(netId_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(146u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_override_microphone_settings(
            hash_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_override_microphone_settings {
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
                        &super::__shared::NumAsU64Arr::into_bytes(hash_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(147u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_horn_enabled(
            vehicle_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_horn_enabled {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(148u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_radio_station_music_only(
            radioStation_: Option<&String>,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_radio_station_music_only {
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
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(149u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_release_named_script_audio_bank(
            audioBank_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_release_named_script_audio_bank {
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
                            super::__internal::send_to_host(&audioBank_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(150u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_release_script_audio_bank() -> altv_wasm_shared::natives_result::ResultOf_release_script_audio_bank {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(151u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_current_playing_speech(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_current_playing_speech {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(152u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ped_voice_group(
            ped_: u32,
            voiceGroupHash_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ped_voice_group {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(voiceGroupHash_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(153u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_update_sound_coord(
            soundId_: i32,
            x_: f32,
            y_: f32,
            z_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_update_sound_coord {
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
                        &super::__shared::NumAsU64Arr::into_bytes(soundId_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(154u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_sound(
            soundId_: i32,
            audioName_: Option<&String>,
            audioRef_: Option<&String>,
            p3_: bool,
            p4_: i32,
            p5_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_sound {
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
                        &super::__shared::NumAsU64Arr::into_bytes(soundId_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&audioName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&audioRef_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p5_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(155u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_cutscene_audio() -> altv_wasm_shared::natives_result::ResultOf_stop_cutscene_audio {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(156u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_audio_is_music_playing() -> altv_wasm_shared::natives_result::ResultOf_audio_is_music_playing {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(157u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_pause_scripted_conversation(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_pause_scripted_conversation {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(158u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_initial_player_station(
            radioStation_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_initial_player_station {
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
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(159u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_stream_from_ped(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_stream_from_ped {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(160u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_entity_for_null_conv_ped(
            p0_: i32,
            entity_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_entity_for_null_conv_ped {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(161u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_interrupt_conversation_and_pause(
            ped_: u32,
            p1_: Option<&String>,
            speaker_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_interrupt_conversation_and_pause {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p1_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&speaker_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(162u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_synchronized_audio_event(
            sceneID_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_synchronized_audio_event {
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
                        &super::__shared::NumAsU64Arr::into_bytes(sceneID_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(163u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ped_interior_walla_density(
            p0_: f32,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ped_interior_walla_density {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(164u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_find_radio_station_index(
            stationNameHash_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_find_radio_station_index {
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
                            stationNameHash_ as u32,
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(165u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_sound_from_coord(
            soundId_: i32,
            audioName_: Option<&String>,
            x_: f32,
            y_: f32,
            z_: f32,
            audioRef_: Option<&String>,
            isNetwork_: bool,
            range_: i32,
            p8_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_sound_from_coord {
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
                        &super::__shared::NumAsU64Arr::into_bytes(soundId_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&audioName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&audioRef_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(isNetwork_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(range_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p8_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(166u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_ped_ambient_speech_native(
            ped_: u32,
            speechName_: Option<&String>,
            speechParam_: Option<&String>,
            p3_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_ped_ambient_speech_native {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&speechName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&speechParam_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(167u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_hint_ambient_audio_bank(
            audioBank_: Option<&String>,
            p1_: bool,
            p2_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_hint_ambient_audio_bank {
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
                            super::__internal::send_to_host(&audioBank_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(168u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_ambient_speech_playing(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_ambient_speech_playing {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(169u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_synchronized_audio_event(
            sceneID_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_synchronized_audio_event {
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
                        &super::__shared::NumAsU64Arr::into_bytes(sceneID_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(170u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_ambient_speech_disabled(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_ambient_speech_disabled {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(171u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_init_synch_scene_audio_with_entity(
            audioEvent_: Option<&String>,
            entity_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_init_synch_scene_audio_with_entity {
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
                            super::__internal::send_to_host(&audioEvent_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(172u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ped_is_drunk(
            ped_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ped_is_drunk {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(173u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_add_ped_to_conversation(
            index_: i32,
            ped_: u32,
            p2_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_add_ped_to_conversation {
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
                        &super::__shared::NumAsU64Arr::into_bytes(index_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p2_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(174u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_skip_to_next_scripted_conversation_line() -> altv_wasm_shared::natives_result::ResultOf_skip_to_next_scripted_conversation_line {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(175u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ambient_zone_list_state(
            ambientZone_: Option<&String>,
            p1_: bool,
            p2_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ambient_zone_list_state {
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
                            super::__internal::send_to_host(&ambientZone_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(176u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_vehicle_force_reverse_warning(
            p0_: i32,
            p1_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_vehicle_force_reverse_warning {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(177u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ambient_voice_name_hash(
            ped_: u32,
            hash_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ambient_voice_name_hash {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(hash_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(178u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_unhint_script_audio_bank() -> altv_wasm_shared::natives_result::ResultOf_unhint_script_audio_bank {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(179u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_restart_scripted_conversation() -> altv_wasm_shared::natives_result::ResultOf_restart_scripted_conversation {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(180u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_vehicle_conversations_persist_new(
            p0_: bool,
            p1_: bool,
            p2_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_vehicle_conversations_persist_new {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(181u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_horn_permanently_on(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_horn_permanently_on {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(182u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_horn_permanently_on_time(
            vehicle_: u32,
            time_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_horn_permanently_on_time {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(time_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(183u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_ped_speaking(
            ped_: u32,
            shaking_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_ped_speaking {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(shaking_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(184u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_horn_active(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_horn_active {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(185u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_prepare_alarm(
            alarmName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_prepare_alarm {
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
                            super::__internal::send_to_host(&alarmName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(186u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_interrupt_conversation(
            ped_: u32,
            voiceline_: Option<&String>,
            speaker_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_interrupt_conversation {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&voiceline_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&speaker_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(187u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_music_oneshot_playing() -> altv_wasm_shared::natives_result::ResultOf_is_music_oneshot_playing {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(188u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_radio_retuning() -> altv_wasm_shared::natives_result::ResultOf_is_radio_retuning {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(189u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_alarm(
            alarmName_: Option<&String>,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_alarm {
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
                            super::__internal::send_to_host(&alarmName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(190u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_sound(
            soundId_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_sound {
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
                        &super::__shared::NumAsU64Arr::into_bytes(soundId_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(191u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_stream() -> altv_wasm_shared::natives_result::ResultOf_stop_stream {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(192u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ped_gender(
            ped_: u32,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ped_gender {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(193u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_player_radio_station_genre() -> altv_wasm_shared::natives_result::ResultOf_get_player_radio_station_genre {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(194u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_audio_script_cleanup_time(
            time_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_audio_script_cleanup_time {
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
                        &super::__shared::NumAsU64Arr::into_bytes(time_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(195u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_radio_to_station_index(
            radioStation_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_radio_to_station_index {
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
                        &super::__shared::NumAsU64Arr::into_bytes(radioStation_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(196u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_unregister_script_with_audio() -> altv_wasm_shared::natives_result::ResultOf_unregister_script_with_audio {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(197u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_block_speech_context_group(
            p0_: Option<&String>,
            p1_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_block_speech_context_group {
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
                            super::__internal::send_to_host(&p0_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(198u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_disable_ped_pain_audio(
            ped_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_disable_ped_pain_audio {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(199u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_variation_chosen_for_scripted_line(
            p0_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_variation_chosen_for_scripted_line {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(200u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_ped_speaking_synced(
            ped_: u32,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_ped_speaking_synced {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(201u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_vehicle_default_horn_ignore_mods(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_vehicle_default_horn_ignore_mods {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(202u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_emitter_radio_station(
            emitterName_: Option<&String>,
            radioStation_: Option<&String>,
            p2_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_emitter_radio_station {
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
                            super::__internal::send_to_host(&emitterName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(203u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_variable_on_sound(
            soundId_: i32,
            unkVariable_: Option<&String>,
            p2_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_variable_on_sound {
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
                        &super::__shared::NumAsU64Arr::into_bytes(soundId_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&unkVariable_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(204u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_mission_complete_audio(
            audioName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_mission_complete_audio {
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
                            super::__internal::send_to_host(&audioName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(205u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_unlock_mission_news_story(
            newsStory_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_unlock_mission_news_story {
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
                        &super::__shared::NumAsU64Arr::into_bytes(newsStory_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(206u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_radio_station_name(
            radioStation_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_radio_station_name {
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
                        &super::__shared::NumAsU64Arr::into_bytes(radioStation_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(207u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_clear_all_broken_glass() -> altv_wasm_shared::natives_result::ResultOf_clear_all_broken_glass {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(208u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_mobile_phone_radio_active() -> altv_wasm_shared::natives_result::ResultOf_is_mobile_phone_radio_active {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(209u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_radio_track(
            radioStation_: Option<&String>,
            radioTrack_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_radio_track {
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
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&radioTrack_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(210u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_remove_portal_settings_override(
            p0_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_remove_portal_settings_override {
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
                            super::__internal::send_to_host(&p0_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(211u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_cancel_all_police_reports() -> altv_wasm_shared::natives_result::ResultOf_cancel_all_police_reports {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(212u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_no_ducking_for_conversation(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_no_ducking_for_conversation {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(213u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_audio_scene_active(
            scene_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_audio_scene_active {
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
                            super::__internal::send_to_host(&scene_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(214u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_microphone_position(
            toggle_: bool,
            x1_: f32,
            y1_: f32,
            z1_: f32,
            x2_: f32,
            y2_: f32,
            z2_: f32,
            x3_: f32,
            y3_: f32,
            z3_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_microphone_position {
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
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x1_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y1_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z1_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x2_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y2_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z2_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x3_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y3_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z3_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(215u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_stream_from_vehicle(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_stream_from_vehicle {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(216u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_enable_stunt_jump_audio() -> altv_wasm_shared::natives_result::ResultOf_enable_stunt_jump_audio {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(217u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_current_playing_ambient_speech(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_current_playing_ambient_speech {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(218u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_audio_flag(
            flagName_: Option<&String>,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_audio_flag {
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
                            super::__internal::send_to_host(&flagName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(219u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_audio_scenes() -> altv_wasm_shared::natives_result::ResultOf_stop_audio_scenes {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(220u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_vehicle_radio_loud(
            vehicle_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_vehicle_radio_loud {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(221u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_pain(
            ped_: u32,
            painID_: i32,
            p1_: i32,
            p3_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_pain {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(painID_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(222u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_variable_on_synch_scene_audio(
            unkVariableName_: Option<&String>,
            value_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_variable_on_synch_scene_audio {
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
                            super::__internal::send_to_host(&unkVariableName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(value_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(223u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ambient_zone_state(
            zoneName_: Option<&String>,
            p1_: bool,
            p2_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ambient_zone_state {
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
                            super::__internal::send_to_host(&zoneName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(224u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_skip_minigun_spin_up_audio(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_skip_minigun_spin_up_audio {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(225u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_mobile_phone_radio_state(
            state_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_mobile_phone_radio_state {
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
                        &super::__shared::NumAsU64Arr::into_bytes(state_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(226u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_use_footstep_script_sweeteners(
            ped_: u32,
            p1_: bool,
            hash_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_use_footstep_script_sweeteners {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(hash_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(227u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_enable_stall_warning_sounds(
            vehicle_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_enable_stall_warning_sounds {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(228u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_veh_forced_radio_this_frame(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_veh_forced_radio_this_frame {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(229u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_radio_auto_unfreeze(
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_radio_auto_unfreeze {
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
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(230u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_animal_vocalization_playing(
            pedHandle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_animal_vocalization_playing {
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
                        &super::__shared::NumAsU64Arr::into_bytes(pedHandle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(231u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_add_line_to_conversation(
            index_: i32,
            p1_: Option<&String>,
            p2_: Option<&String>,
            p3_: i32,
            p4_: i32,
            p5_: bool,
            p6_: bool,
            p7_: bool,
            p8_: bool,
            p9_: i32,
            p10_: bool,
            p11_: bool,
            p12_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_add_line_to_conversation {
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
                        &super::__shared::NumAsU64Arr::into_bytes(index_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p1_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p2_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p5_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p6_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p7_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p8_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p9_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p10_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p11_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p12_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(232u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_next_audible_beat(
            out1_: f32,
            out2_: f32,
            out3_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_next_audible_beat {
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
                        &super::__shared::NumAsU64Arr::into_bytes(out1_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(out2_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(out3_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(233u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_ped_ambient_speech_and_clone_native(
            ped_: u32,
            speechName_: Option<&String>,
            speechParam_: Option<&String>,
            p3_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_ped_ambient_speech_and_clone_native {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&speechName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&speechParam_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(234u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_radio_to_station_name(
            stationName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_radio_to_station_name {
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
                            super::__internal::send_to_host(&stationName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(235u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_register_script_with_audio(
            p0_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_register_script_with_audio {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(236u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_prepare_synchronized_audio_event(
            audioEvent_: Option<&String>,
            p1_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_prepare_synchronized_audio_event {
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
                            super::__internal::send_to_host(&audioEvent_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(237u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_mobile_interference_active() -> altv_wasm_shared::natives_result::ResultOf_is_mobile_interference_active {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(238u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_init_synch_scene_audio_with_position(
            audioEvent_: Option<&String>,
            x_: f32,
            y_: f32,
            z_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_init_synch_scene_audio_with_position {
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
                            super::__internal::send_to_host(&audioEvent_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(239u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_preload_vehicle_audio_bank(
            vehicleModel_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_preload_vehicle_audio_bank {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicleModel_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(240u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_deferred_sound_frontend(
            soundName_: Option<&String>,
            soundsetName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_deferred_sound_frontend {
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
                            super::__internal::send_to_host(&soundName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&soundsetName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(241u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_animal_mood(
            animal_: u32,
            mood_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_animal_mood {
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
                        &super::__shared::NumAsU64Arr::into_bytes(animal_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(mood_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(242u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_scripted_speech_playing(
            p0_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_scripted_speech_playing {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(243u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_end_credits_music(
            play_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_end_credits_music {
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
                        &super::__shared::NumAsU64Arr::into_bytes(play_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(244u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_release_weapon_audio() -> altv_wasm_shared::natives_result::ResultOf_release_weapon_audio {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(245u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_activate_audio_slowmo_mode(
            mode_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_activate_audio_slowmo_mode {
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
                            super::__internal::send_to_host(&mode_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(246u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_stream_playing() -> altv_wasm_shared::natives_result::ResultOf_is_stream_playing {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(247u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_create_new_scripted_conversation() -> altv_wasm_shared::natives_result::ResultOf_create_new_scripted_conversation {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(248u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_override_player_ground_material(
            hash_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_override_player_ground_material {
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
                        &super::__shared::NumAsU64Arr::into_bytes(hash_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(249u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_reset_vehicle_startup_rev_sound(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_reset_vehicle_startup_rev_sound {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(250u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_vehicle_horn_sound_index(
            vehicle_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_vehicle_horn_sound_index {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(251u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_freeze_microphone() -> altv_wasm_shared::natives_result::ResultOf_freeze_microphone {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(252u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_scripted_conversation(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_scripted_conversation {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(253u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_positioned_player_vehicle_radio_emitter_enabled(
            p0_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_positioned_player_vehicle_radio_emitter_enabled {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(254u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_radio_retune_down() -> altv_wasm_shared::natives_result::ResultOf_set_radio_retune_down {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(255u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_deactivate_audio_slowmo_mode(
            mode_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_deactivate_audio_slowmo_mode {
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
                            super::__internal::send_to_host(&mode_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(256u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_scripted_conversation_loaded() -> altv_wasm_shared::natives_result::ResultOf_is_scripted_conversation_loaded {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(257u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_audio_scene(
            scene_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_audio_scene {
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
                            super::__internal::send_to_host(&scene_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(258u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_police_report(
            name_: Option<&String>,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_police_report {
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
                            super::__internal::send_to_host(&name_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(259u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_smoke_grenade_explosion_sounds() -> altv_wasm_shared::natives_result::ResultOf_stop_smoke_grenade_explosion_sounds {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(260u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_audio_vehicle_priority(
            vehicle_: u32,
            p1_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_audio_vehicle_priority {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(261u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_sound_from_entity(
            soundId_: i32,
            audioName_: Option<&String>,
            entity_: u32,
            audioRef_: Option<&String>,
            isNetwork_: bool,
            p5_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_sound_from_entity {
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
                        &super::__shared::NumAsU64Arr::into_bytes(soundId_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&audioName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&audioRef_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(isNetwork_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p5_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(262u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_is_preloaded_conversation_ready() -> altv_wasm_shared::natives_result::ResultOf_get_is_preloaded_conversation_ready {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(263u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_reset_trevor_rage() -> altv_wasm_shared::natives_result::ResultOf_reset_trevor_rage {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(264u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_music_playtime() -> altv_wasm_shared::natives_result::ResultOf_get_music_playtime {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(265u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_player_radio_station_index() -> altv_wasm_shared::natives_result::ResultOf_get_player_radio_station_index {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(266u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_player_angry(
            ped_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_player_angry {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(267u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_stream_from_object(
            object_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_stream_from_object {
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
                        &super::__shared::NumAsU64Arr::into_bytes(object_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(268u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_ambient_speech_from_position_native(
            speechName_: Option<&String>,
            voiceName_: Option<&String>,
            x_: f32,
            y_: f32,
            z_: f32,
            speechParam_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_ambient_speech_from_position_native {
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
                            super::__internal::send_to_host(&speechName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&voiceName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&speechParam_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(269u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_animal_vocalization(
            pedHandle_: u32,
            p1_: i32,
            speechName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_animal_vocalization {
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
                        &super::__shared::NumAsU64Arr::into_bytes(pedHandle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&speechName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(270u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_audio_scene_variable(
            scene_: Option<&String>,
            unkVariable_: Option<&String>,
            value_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_audio_scene_variable {
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
                            super::__internal::send_to_host(&scene_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&unkVariable_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(value_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(271u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_block_death_jingle(
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_block_death_jingle {
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
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(272u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_num_unlocked_radio_stations() -> altv_wasm_shared::natives_result::ResultOf_get_num_unlocked_radio_stations {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(273u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_vehicle_startup_rev_sound(
            vehicle_: u32,
            p1_: Option<&String>,
            p2_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_vehicle_startup_rev_sound {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p1_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p2_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(274u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_override_underwater_stream(
            p0_: Option<&String>,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_override_underwater_stream {
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
                            super::__internal::send_to_host(&p0_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(275u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_vehicle_missile_warning_enabled(
            vehicle_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_vehicle_missile_warning_enabled {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(276u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_ambient_zone_list_state_persistent(
            ambientZone_: Option<&String>,
            p1_: bool,
            p2_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_ambient_zone_list_state_persistent {
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
                            super::__internal::send_to_host(&ambientZone_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(277u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_reset_ped_audio_flags(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_reset_ped_audio_flags {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(278u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_siren_bypass_mp_driver_check(
            vehicle_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_siren_bypass_mp_driver_check {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(279u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_player_radio_station_name() -> altv_wasm_shared::natives_result::ResultOf_get_player_radio_station_name {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(280u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_frontend_radio_active(
            active_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_frontend_radio_active {
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
                        &super::__shared::NumAsU64Arr::into_bytes(active_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(281u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_block_all_speech_from_ped(
            ped_: u32,
            p1_: bool,
            p2_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_block_all_speech_from_ped {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(282u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_ped_ringtone(
            ringtoneName_: Option<&String>,
            ped_: u32,
            p2_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_ped_ringtone {
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
                            super::__internal::send_to_host(&ringtoneName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(283u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_use_siren_as_horn(
            vehicle_: u32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_use_siren_as_horn {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(284u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_hint_script_audio_bank(
            audioBank_: Option<&String>,
            p1_: bool,
            p2_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_hint_script_audio_bank {
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
                            super::__internal::send_to_host(&audioBank_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(285u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_record_broken_glass(
            x_: f32,
            y_: f32,
            z_: f32,
            radius_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_record_broken_glass {
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
                        &super::__shared::NumAsU64Arr::into_bytes(x_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(radius_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(286u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_unfreeze_radio_station(
            radioStation_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_unfreeze_radio_station {
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
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(287u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_has_sound_finished(
            soundId_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_has_sound_finished {
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
                        &super::__shared::NumAsU64Arr::into_bytes(soundId_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(288u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_request_ambient_audio_bank(
            audioBank_: Option<&String>,
            p1_: bool,
            p2_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_request_ambient_audio_bank {
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
                            super::__internal::send_to_host(&audioBank_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(289u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_radio_retune_up() -> altv_wasm_shared::natives_result::ResultOf_set_radio_retune_up {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(290u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_lock_radio_station_track_list(
            radioStation_: Option<&String>,
            trackListName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_lock_radio_station_track_list {
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
                            super::__internal::send_to_host(&radioStation_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&trackListName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(291u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_reactivate_all_world_brains_that_are_waiting_till_out_of_range() -> altv_wasm_shared::natives_result::ResultOf_reactivate_all_world_brains_that_are_waiting_till_out_of_range {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(292u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_register_object_script_brain(
            scriptName_: Option<&String>,
            modelHash_: u32,
            p2_: i32,
            activationRange_: f32,
            p4_: i32,
            p5_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_register_object_script_brain {
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
                            super::__internal::send_to_host(&scriptName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(modelHash_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            activationRange_ as f32,
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p5_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(293u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_disable_script_brain_set(
            brainSet_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_disable_script_brain_set {
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
                        &super::__shared::NumAsU64Arr::into_bytes(brainSet_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(294u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_register_world_point_script_brain(
            scriptName_: Option<&String>,
            activationRange_: f32,
            p2_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_register_world_point_script_brain {
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
                            super::__internal::send_to_host(&scriptName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            activationRange_ as f32,
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(295u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_reactivate_all_object_brains_that_are_waiting_till_out_of_range() -> altv_wasm_shared::natives_result::ResultOf_reactivate_all_object_brains_that_are_waiting_till_out_of_range {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(296u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_add_script_to_random_ped(
            name_: Option<&String>,
            model_: u32,
            p2_: f32,
            p3_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_add_script_to_random_ped {
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
                            super::__internal::send_to_host(&name_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(model_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(297u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_enable_script_brain_set(
            brainSet_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_enable_script_brain_set {
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
                        &super::__shared::NumAsU64Arr::into_bytes(brainSet_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(298u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_reactivate_named_world_brains_waiting_till_out_of_range(
            scriptName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_reactivate_named_world_brains_waiting_till_out_of_range {
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
                            super::__internal::send_to_host(&scriptName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(299u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_reactivate_named_object_brains_waiting_till_out_of_range(
            scriptName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_reactivate_named_object_brains_waiting_till_out_of_range {
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
                            super::__internal::send_to_host(&scriptName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(300u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_world_point_within_brain_activation_range() -> altv_wasm_shared::natives_result::ResultOf_is_world_point_within_brain_activation_range {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(301u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_object_within_brain_activation_range(
            object_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_object_within_brain_activation_range {
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
                        &super::__shared::NumAsU64Arr::into_bytes(object_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(302u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_gameplay_cam_shaking() -> altv_wasm_shared::natives_result::ResultOf_is_gameplay_cam_shaking {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(303u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_cam_max_motion_blur_strength_this_update(
            p0_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_cam_max_motion_blur_strength_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(304u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_active(
            cam_: i32,
            active_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_active {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(active_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(305u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_cam_spline_paused(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_cam_spline_paused {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(306u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_cam_rendering(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_cam_rendering {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(307u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_cam_interpolating(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_cam_interpolating {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(308u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_cam_dof_strength(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_cam_dof_strength {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(309u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_render_script_cams(
            render_: bool,
            ease_: bool,
            easeTime_: i32,
            p3_: bool,
            p4_: bool,
            p5_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_render_script_cams {
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
                        &super::__shared::NumAsU64Arr::into_bytes(render_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(ease_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(easeTime_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p5_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(310u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_add_cam_spline_node_using_camera_frame(
            cam_: i32,
            cam2_: i32,
            length_: i32,
            p3_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_add_cam_spline_node_using_camera_frame {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(cam2_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(length_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(311u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_force_vehicle_cam_stunt_settings_this_update() -> altv_wasm_shared::natives_result::ResultOf_force_vehicle_cam_stunt_settings_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(312u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_first_person_aim_cam_near_clip_this_update(
            p0_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_first_person_aim_cam_near_clip_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(313u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_gameplay_cam_shaking(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_gameplay_cam_shaking {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(314u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_add_cam_spline_node_using_camera(
            cam_: i32,
            cam2_: i32,
            length_: i32,
            p3_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_add_cam_spline_node_using_camera {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(cam2_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(length_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(315u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_first_person_shooter_camera_heading(
            yaw_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_first_person_shooter_camera_heading {
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
                        &super::__shared::NumAsU64Arr::into_bytes(yaw_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(316u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_first_person_flash_effect_vehicle_model_hash(
            vehicleModel_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_first_person_flash_effect_vehicle_model_hash {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicleModel_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(317u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cutscene_cam_far_clip_this_update(
            p0_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cutscene_cam_far_clip_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(318u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_spline_duration(
            cam_: i32,
            timeDuration_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_spline_duration {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(timeDuration_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(319u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_hard_attach_cam_to_ped_bone(
            cam_: i32,
            ped_: u32,
            boneIndex_: i32,
            p3_: f32,
            p4_: f32,
            p5_: f32,
            p6_: f32,
            p7_: f32,
            p8_: f32,
            p9_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_hard_attach_cam_to_ped_bone {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(boneIndex_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p5_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p6_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p7_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p8_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p9_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(320u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_gameplay_cam_coord() -> altv_wasm_shared::natives_result::ResultOf_get_gameplay_cam_coord {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(321u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_final_rendered_cam_motion_blur_strength() -> altv_wasm_shared::natives_result::ResultOf_get_final_rendered_cam_motion_blur_strength {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(322u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_use_shallow_dof_mode(
            cam_: i32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_use_shallow_dof_mode {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(323u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_disable_cinematic_slow_mo_this_update() -> altv_wasm_shared::natives_result::ResultOf_disable_cinematic_slow_mo_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(324u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_entity_hint(
            entity_: u32,
            xOffset_: f32,
            yOffset_: f32,
            zOffset_: f32,
            p4_: bool,
            time_: i32,
            easeInTime_: i32,
            easeOutTime_: i32,
            p8_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_entity_hint {
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
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(xOffset_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(yOffset_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(zOffset_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(time_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(easeInTime_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(easeOutTime_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p8_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(325u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_follow_vehicle_cam_zoom_level(
            zoomLevel_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_follow_vehicle_cam_zoom_level {
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
                        &super::__shared::NumAsU64Arr::into_bytes(zoomLevel_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(326u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_cam_active_view_mode_context() -> altv_wasm_shared::natives_result::ResultOf_get_cam_active_view_mode_context {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(327u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_disable_aim_cam_this_update() -> altv_wasm_shared::natives_result::ResultOf_disable_aim_cam_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(328u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_debug_name(
            camera_: i32,
            name_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_debug_name {
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
                        &super::__shared::NumAsU64Arr::into_bytes(camera_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&name_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(329u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_script_global_shaking(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_script_global_shaking {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(330u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_in_vehicle_mobile_phone_camera_rendering() -> altv_wasm_shared::natives_result::ResultOf_is_in_vehicle_mobile_phone_camera_rendering {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(331u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_hard_attach_cam_to_entity(
            cam_: i32,
            entity_: u32,
            xRot_: f32,
            yRot_: f32,
            zRot_: f32,
            xOffset_: f32,
            yOffset_: f32,
            zOffset_: f32,
            isRelative_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_hard_attach_cam_to_entity {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(xRot_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(yRot_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(zRot_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(xOffset_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(yOffset_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(zOffset_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(isRelative_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(332u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_first_person_flash_effect_vehicle_model_name(
            vehicleName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_first_person_flash_effect_vehicle_model_name {
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
                            super::__internal::send_to_host(&vehicleName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(333u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_cinematic_cam_shaking(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_cinematic_cam_shaking {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(334u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_spline_phase(
            cam_: i32,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_spline_phase {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(335u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_code_gameplay_hint(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_code_gameplay_hint {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(336u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_cam_far_dof(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_cam_far_dof {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(337u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_final_rendered_remote_player_cam_rot(
            player_: u32,
            rotationOrder_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_final_rendered_remote_player_cam_rot {
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
                        &super::__shared::NumAsU64Arr::into_bytes(player_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotationOrder_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(338u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_allow_motion_blur_decay(
            p0_: i32,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_allow_motion_blur_decay {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(339u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_use_script_cam_for_ambient_population_origin_this_frame(
            p0_: bool,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_use_script_cam_for_ambient_population_origin_this_frame {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(340u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_force_bonnet_camera_relative_heading_and_pitch(
            p0_: f32,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_force_bonnet_camera_relative_heading_and_pitch {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(341u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_view_mode_for_context(
            context_: i32,
            viewMode_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_view_mode_for_context {
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
                        &super::__shared::NumAsU64Arr::into_bytes(context_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(viewMode_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(342u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_cam_ignore_entity_collision_this_update(
            entity_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_cam_ignore_entity_collision_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(343u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_ped_hint(
            ped_: u32,
            x1_: f32,
            y1_: f32,
            z1_: f32,
            p4_: bool,
            duration_: i32,
            blendOutDuration_: i32,
            blendInDuration_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_ped_hint {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x1_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y1_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z1_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(duration_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            blendOutDuration_ as i32,
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            blendInDuration_ as i32,
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(344u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_dof_max_near_in_focus_distance_blend_level(
            camera_: i32,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_dof_max_near_in_focus_distance_blend_level {
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
                        &super::__shared::NumAsU64Arr::into_bytes(camera_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(345u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_first_person_aim_cam_relative_heading_limits_this_update(
            p0_: f32,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_first_person_aim_cam_relative_heading_limits_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(346u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_interpolating_from_script_cams() -> altv_wasm_shared::natives_result::ResultOf_is_interpolating_from_script_cams {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(347u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_cutscene_cam_shaking(
            p0_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_cutscene_cam_shaking {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(348u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_follow_ped_cam_zoom_level() -> altv_wasm_shared::natives_result::ResultOf_get_follow_ped_cam_zoom_level {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(349u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_force_tightspace_custom_framing_this_update() -> altv_wasm_shared::natives_result::ResultOf_force_tightspace_custom_framing_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(350u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_gameplay_cam_rendering() -> altv_wasm_shared::natives_result::ResultOf_is_gameplay_cam_rendering {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(351u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_gameplay_cam_relative_pitch() -> altv_wasm_shared::natives_result::ResultOf_get_gameplay_cam_relative_pitch {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(352u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_dof_planes(
            cam_: i32,
            p1_: f32,
            p2_: f32,
            p3_: f32,
            p4_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_dof_planes {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(353u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_near_dof(
            cam_: i32,
            nearDOF_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_near_dof {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(nearDOF_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(354u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_allow_custom_vehicle_drive_by_cam_this_update(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_allow_custom_vehicle_drive_by_cam_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(355u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_override_cam_spline_velocity(
            cam_: i32,
            p1_: i32,
            p2_: f32,
            p3_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_override_cam_spline_velocity {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(356u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_anim_current_phase(
            cam_: i32,
            phase_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_anim_current_phase {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(phase_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(357u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_third_person_aim_cam_near_clip_this_update(
            p0_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_third_person_aim_cam_near_clip_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(358u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_use_dedicated_stunt_camera_this_update(
            camName_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_use_dedicated_stunt_camera_this_update {
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
                            super::__internal::send_to_host(&camName_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(359u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_follow_ped_cam_this_update(
            camName_: Option<&String>,
            p1_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_follow_ped_cam_this_update {
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
                            super::__internal::send_to_host(&camName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(360u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_inherit_roll_vehicle(
            cam_: i32,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_inherit_roll_vehicle {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(361u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_scripted_camera_is_first_person_this_frame(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_scripted_camera_is_first_person_this_frame {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(362u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_dof_focal_length_multiplier(
            camera_: i32,
            multiplier_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_dof_focal_length_multiplier {
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
                        &super::__shared::NumAsU64Arr::into_bytes(camera_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(multiplier_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(363u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_force_camera_relative_heading_and_pitch(
            roll_: f32,
            pitch_: f32,
            yaw_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_force_camera_relative_heading_and_pitch {
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
                        &super::__shared::NumAsU64Arr::into_bytes(roll_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(pitch_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(yaw_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(364u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_are_widescreen_borders_active() -> altv_wasm_shared::natives_result::ResultOf_are_widescreen_borders_active {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(365u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_cam_motion_blur_scaling_this_update(
            p0_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_cam_motion_blur_scaling_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(366u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_disable_cam_collision_for_object(
            entity_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_disable_cam_collision_for_object {
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
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(367u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_coord(
            cam_: i32,
            posX_: f32,
            posY_: f32,
            posZ_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_coord {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(posX_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(posY_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(posZ_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(368u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_cinematic_first_person_vehicle_interior_cam_rendering() -> altv_wasm_shared::natives_result::ResultOf_is_cinematic_first_person_vehicle_interior_cam_rendering {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(369u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_fly_cam_horizontal_response(
            cam_: i32,
            p1_: f32,
            p2_: f32,
            p3_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_fly_cam_horizontal_response {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(370u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_hint_fov(
            FOV_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_hint_fov {
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
                        &super::__shared::NumAsU64Arr::into_bytes(FOV_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(371u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cinematic_button_active(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cinematic_button_active {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(372u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_rendering_cam() -> altv_wasm_shared::natives_result::ResultOf_get_rendering_cam {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(373u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_point_cam_at_entity(
            cam_: i32,
            entity_: u32,
            p2_: f32,
            p3_: f32,
            p4_: f32,
            p5_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_point_cam_at_entity {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p5_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(374u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_disable_first_person_flash_effect_this_update() -> altv_wasm_shared::natives_result::ResultOf_disable_first_person_flash_effect_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(375u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_disable_near_clip_scan_this_update() -> altv_wasm_shared::natives_result::ResultOf_disable_near_clip_scan_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(376u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_follow_ped_cam_view_mode(
            viewMode_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_follow_ped_cam_view_mode {
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
                        &super::__shared::NumAsU64Arr::into_bytes(viewMode_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(377u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_screen_faded_in() -> altv_wasm_shared::natives_result::ResultOf_is_screen_faded_in {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(378u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_final_rendered_cam_rot(
            rotationOrder_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_final_rendered_cam_rot {
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
                        &super::__shared::NumAsU64Arr::into_bytes(rotationOrder_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(379u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_first_person_flash_effect_type(
            p0_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_first_person_flash_effect_type {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(380u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_was_fly_cam_constrained_on_previous_udpate(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_was_fly_cam_constrained_on_previous_udpate {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(381u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_screen_fading_in() -> altv_wasm_shared::natives_result::ResultOf_is_screen_fading_in {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(382u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_follow_vehicle_cam_seat_this_update(
            seatIndex_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_follow_vehicle_cam_seat_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(seatIndex_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(383u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_hint_camera_relative_side_offset(
            xOffset_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_hint_camera_relative_side_offset {
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
                        &super::__shared::NumAsU64Arr::into_bytes(xOffset_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(384u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_trigger_vehicle_part_broken_camera_shake(
            vehicle_: u32,
            p1_: i32,
            p2_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_trigger_vehicle_part_broken_camera_shake {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(385u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_first_person_aim_cam_active() -> altv_wasm_shared::natives_result::ResultOf_is_first_person_aim_cam_active {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(386u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_create_camera(
            camHash_: u32,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_create_camera {
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
                        &super::__shared::NumAsU64Arr::into_bytes(camHash_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(387u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_dof_strength(
            cam_: i32,
            dofStrength_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_dof_strength {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(dofStrength_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(388u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_final_rendered_remote_player_cam_fov(
            player_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_final_rendered_remote_player_cam_fov {
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
                        &super::__shared::NumAsU64Arr::into_bytes(player_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(389u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_add_cam_spline_node_using_gameplay_frame(
            cam_: i32,
            length_: i32,
            p2_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_add_cam_spline_node_using_gameplay_frame {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(length_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(390u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_attach_cam_to_ped_bone(
            cam_: i32,
            ped_: u32,
            boneIndex_: i32,
            x_: f32,
            y_: f32,
            z_: f32,
            heading_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_attach_cam_to_ped_bone {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(boneIndex_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(heading_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(391u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_camera_prevent_collision_settings_for_triplehead_in_interiors_this_update() -> altv_wasm_shared::natives_result::ResultOf_camera_prevent_collision_settings_for_triplehead_in_interiors_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(392u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_disable_cinematic_vehicle_idle_mode_this_update() -> altv_wasm_shared::natives_result::ResultOf_disable_cinematic_vehicle_idle_mode_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(393u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_use_vehicle_cam_stunt_settings_this_update() -> altv_wasm_shared::natives_result::ResultOf_use_vehicle_cam_stunt_settings_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(394u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_gameplay_cam_fov() -> altv_wasm_shared::natives_result::ResultOf_get_gameplay_cam_fov {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(395u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_controls_mini_map_heading(
            cam_: i32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_controls_mini_map_heading {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(396u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_point_cam_at_ped_bone(
            cam_: i32,
            ped_: u32,
            boneIndex_: i32,
            x_: f32,
            y_: f32,
            z_: f32,
            p6_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_point_cam_at_ped_bone {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(boneIndex_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p6_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(397u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_aim_cam_active() -> altv_wasm_shared::natives_result::ResultOf_is_aim_cam_active {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(398u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_shake_cam(
            cam_: i32,
            type_: Option<&String>,
            amplitude_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_shake_cam {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&type_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(amplitude_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(399u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_create_camera_with_params(
            camHash_: u32,
            posX_: f32,
            posY_: f32,
            posZ_: f32,
            rotX_: f32,
            rotY_: f32,
            rotZ_: f32,
            fov_: f32,
            p8_: bool,
            p9_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_create_camera_with_params {
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
                        &super::__shared::NumAsU64Arr::into_bytes(camHash_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(posX_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(posY_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(posZ_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotX_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotY_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotZ_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(fov_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p8_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p9_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(400u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_cam_shaking(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_cam_shaking {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(401u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_cam_relative_pitch(
            angle_: f32,
            scalingFactor_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_cam_relative_pitch {
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
                        &super::__shared::NumAsU64Arr::into_bytes(angle_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(scalingFactor_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(402u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_motion_blur_strength(
            cam_: i32,
            strength_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_motion_blur_strength {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(strength_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(403u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_interpolating_to_script_cams() -> altv_wasm_shared::natives_result::ResultOf_is_interpolating_to_script_cams {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(404u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_first_person_aim_cam_zoom_factor(
            zoomFactor_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_first_person_aim_cam_zoom_factor {
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
                        &super::__shared::NumAsU64Arr::into_bytes(zoomFactor_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(405u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_gameplay_cam_looking_behind() -> altv_wasm_shared::natives_result::ResultOf_is_gameplay_cam_looking_behind {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(406u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_reset_gameplay_cam_full_attach_parent_transform_timer() -> altv_wasm_shared::natives_result::ResultOf_reset_gameplay_cam_full_attach_parent_transform_timer {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(407u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_use_hi_dof_on_synced_scene_this_update() -> altv_wasm_shared::natives_result::ResultOf_set_use_hi_dof_on_synced_scene_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(408u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_create_cinematic_shot(
            p0_: u32,
            time_: i32,
            p2_: bool,
            entity_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_create_cinematic_shot {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(time_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(entity_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(409u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_gameplay_cam_relative_heading() -> altv_wasm_shared::natives_result::ResultOf_get_gameplay_cam_relative_heading {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(410u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_aim_cam_active_in_accurate_mode() -> altv_wasm_shared::natives_result::ResultOf_is_aim_cam_active_in_accurate_mode {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(411u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_first_person_shooter_camera_pitch(
            pitch_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_first_person_shooter_camera_pitch {
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
                        &super::__shared::NumAsU64Arr::into_bytes(pitch_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(412u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_cinematic_shot(
            p0_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_cinematic_shot {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(413u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_debug_cam() -> altv_wasm_shared::natives_result::ResultOf_get_debug_cam {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(414u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_screen_fading_out() -> altv_wasm_shared::natives_result::ResultOf_is_screen_fading_out {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(415u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_table_games_camera_this_update(
            hash_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_table_games_camera_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(hash_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(416u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_ignore_menu_preference_for_bonnet_camera_this_update() -> altv_wasm_shared::natives_result::ResultOf_ignore_menu_preference_for_bonnet_camera_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(417u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_spline_node_extra_flags(
            cam_: i32,
            p1_: i32,
            flags_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_spline_node_extra_flags {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(flags_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(418u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_cam_rot(
            cam_: i32,
            rotationOrder_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_cam_rot {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotationOrder_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(419u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_override_cam_spline_motion_blur(
            cam_: i32,
            p1_: i32,
            p2_: f32,
            p3_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_override_cam_spline_motion_blur {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(420u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_dof_fnumber_of_lens(
            camera_: i32,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_dof_fnumber_of_lens {
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
                        &super::__shared::NumAsU64Arr::into_bytes(camera_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(421u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_first_person_aim_cam_zoom_factor() -> altv_wasm_shared::natives_result::ResultOf_get_first_person_aim_cam_zoom_factor {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(422u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_death_fail_effect_state(
            p0_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_death_fail_effect_state {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(423u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_final_rendered_cam_fov() -> altv_wasm_shared::natives_result::ResultOf_get_final_rendered_cam_fov {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(424u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_gameplay_cam_rot(
            rotationOrder_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_gameplay_cam_rot {
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
                        &super::__shared::NumAsU64Arr::into_bytes(rotationOrder_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(425u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_spline_node_ease(
            cam_: i32,
            easingFunction_: i32,
            p2_: i32,
            p3_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_spline_node_ease {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(easingFunction_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(426u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_object_hint(
            object_: u32,
            xOffset_: f32,
            yOffset_: f32,
            zOffset_: f32,
            p4_: bool,
            time_: i32,
            easeInTime_: i32,
            easeOutTime_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_object_hint {
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
                        &super::__shared::NumAsU64Arr::into_bytes(object_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(xOffset_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(yOffset_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(zOffset_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(time_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(easeInTime_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(easeOutTime_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(427u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_rot(
            cam_: i32,
            rotX_: f32,
            rotY_: f32,
            rotZ_: f32,
            rotationOrder_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_rot {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotX_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotY_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotZ_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotationOrder_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(428u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_add_cam_spline_node(
            camera_: i32,
            x_: f32,
            y_: f32,
            z_: f32,
            xRot_: f32,
            yRot_: f32,
            zRot_: f32,
            length_: i32,
            smoothingStyle_: i32,
            rotationOrder_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_add_cam_spline_node {
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
                        &super::__shared::NumAsU64Arr::into_bytes(camera_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(xRot_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(yRot_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(zRot_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(length_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(smoothingStyle_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotationOrder_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(429u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_destroy_cam(
            cam_: i32,
            bScriptHostCam_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_destroy_cam {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(bScriptHostCam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(430u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_do_screen_fade_out(
            duration_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_do_screen_fade_out {
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
                        &super::__shared::NumAsU64Arr::into_bytes(duration_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(431u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_focus_ped_on_screen(
            p0_: f32,
            p1_: i32,
            p2_: f32,
            p3_: f32,
            p4_: f32,
            p5_: f32,
            p6_: f32,
            p7_: i32,
            p8_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_focus_ped_on_screen {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p5_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p6_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p7_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p8_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(432u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_cam_follow_ped_this_update(
            ped_: u32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_cam_follow_ped_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(ped_ as u32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(433u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_replay_get_max_distance_allowed_from_player() -> altv_wasm_shared::natives_result::ResultOf_replay_get_max_distance_allowed_from_player {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(434u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_affects_aiming(
            cam_: i32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_affects_aiming {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(435u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_follow_ped_cam_view_mode() -> altv_wasm_shared::natives_result::ResultOf_get_follow_ped_cam_view_mode {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(436u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_attach_cam_to_vehicle_bone(
            cam_: i32,
            vehicle_: u32,
            boneIndex_: i32,
            relativeRotation_: bool,
            rotX_: f32,
            rotY_: f32,
            rotZ_: f32,
            offsetX_: f32,
            offsetY_: f32,
            offsetZ_: f32,
            fixedDirection_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_attach_cam_to_vehicle_bone {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(boneIndex_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            relativeRotation_ as i32,
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotX_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotY_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotZ_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(offsetX_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(offsetY_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(offsetZ_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(fixedDirection_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(437u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_destroy_all_cams(
            bScriptHostCam_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_destroy_all_cams {
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
                        &super::__shared::NumAsU64Arr::into_bytes(bScriptHostCam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(438u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_third_person_cam_relative_heading_limits_this_update(
            minimum_: f32,
            maximum_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_third_person_cam_relative_heading_limits_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(minimum_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(maximum_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(439u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_follow_vehicle_cam_high_angle_mode_this_update(
            p0_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_follow_vehicle_cam_high_angle_mode_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(440u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_final_rendered_cam_far_dof() -> altv_wasm_shared::natives_result::ResultOf_get_final_rendered_cam_far_dof {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(441u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_play_cam_anim(
            cam_: i32,
            animName_: Option<&String>,
            animDictionary_: Option<&String>,
            x_: f32,
            y_: f32,
            z_: f32,
            xRot_: f32,
            yRot_: f32,
            zRot_: f32,
            p9_: bool,
            p10_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_play_cam_anim {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&animName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&animDictionary_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(xRot_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(yRot_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(zRot_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p9_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p10_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(442u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_follow_vehicle_cam_high_angle_mode_every_update(
            p0_: bool,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_follow_vehicle_cam_high_angle_mode_every_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(443u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_invalidate_cinematic_vehicle_idle_mode() -> altv_wasm_shared::natives_result::ResultOf_invalidate_cinematic_vehicle_idle_mode {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(444u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_block_first_person_orientation_reset_this_update() -> altv_wasm_shared::natives_result::ResultOf_block_first_person_orientation_reset_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(445u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_active_with_interp(
            camTo_: i32,
            camFrom_: i32,
            duration_: i32,
            easeLocation_: i32,
            easeRotation_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_active_with_interp {
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
                        &super::__shared::NumAsU64Arr::into_bytes(camTo_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(camFrom_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(duration_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(easeLocation_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(easeRotation_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(446u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_final_rendered_cam_near_dof() -> altv_wasm_shared::natives_result::ResultOf_get_final_rendered_cam_near_dof {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(447u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_cam_anim_current_phase(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_cam_anim_current_phase {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(448u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_use_hi_dof() -> altv_wasm_shared::natives_result::ResultOf_set_use_hi_dof {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(449u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_final_rendered_cam_coord() -> altv_wasm_shared::natives_result::ResultOf_get_final_rendered_cam_coord {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(450u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_vehicle_hint(
            vehicle_: u32,
            offsetX_: f32,
            offsetY_: f32,
            offsetZ_: f32,
            p4_: bool,
            time_: i32,
            easeInTime_: i32,
            easeOutTime_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_vehicle_hint {
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
                        &super::__shared::NumAsU64Arr::into_bytes(vehicle_ as u32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(offsetX_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(offsetY_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(offsetZ_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p4_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(time_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(easeInTime_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(easeOutTime_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(451u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_animated_shake_cam(
            cam_: i32,
            p1_: Option<&String>,
            p2_: Option<&String>,
            p3_: Option<&String>,
            amplitude_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_animated_shake_cam {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p1_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p2_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p3_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(amplitude_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(452u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_is_inside_vehicle(
            cam_: i32,
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_is_inside_vehicle {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(453u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_detach_cam(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_detach_cam {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(454u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_force_cinematic_rendering_this_update(
            toggle_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_force_cinematic_rendering_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(toggle_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(455u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_follow_vehicle_cam_view_mode() -> altv_wasm_shared::natives_result::ResultOf_get_follow_vehicle_cam_view_mode {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(456u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_third_person_cam_relative_pitch_limits_this_update(
            minimum_: f32,
            maximum_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_third_person_cam_relative_pitch_limits_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(minimum_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(maximum_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(457u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_spline_node_velocity_scale(
            cam_: i32,
            p1_: i32,
            scale_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_spline_node_velocity_scale {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(scale_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(458u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_bypass_camera_collision_buoyancy_test_this_update() -> altv_wasm_shared::natives_result::ResultOf_bypass_camera_collision_buoyancy_test_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(459u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_does_cam_exist(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_does_cam_exist {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(460u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_cam_shake_amplitude(
            amplitude_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_cam_shake_amplitude {
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
                        &super::__shared::NumAsU64Arr::into_bytes(amplitude_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(461u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_force_cam_far_clip(
            cam_: i32,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_force_cam_far_clip {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(462u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_follow_vehicle_cam_view_mode(
            viewMode_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_follow_vehicle_cam_view_mode {
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
                        &super::__shared::NumAsU64Arr::into_bytes(viewMode_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(463u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_disable_cinematic_bonnet_camera_this_update() -> altv_wasm_shared::natives_result::ResultOf_disable_cinematic_bonnet_camera_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(464u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_far_clip(
            cam_: i32,
            farClip_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_far_clip {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(farClip_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(465u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_disable_first_person_camera_water_clipping_test_this_update() -> altv_wasm_shared::natives_result::ResultOf_disable_first_person_camera_water_clipping_test_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(466u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_fov(
            cam_: i32,
            fieldOfView_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_fov {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(fieldOfView_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(467u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_cinematic_cam_rendering() -> altv_wasm_shared::natives_result::ResultOf_is_cinematic_cam_rendering {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(468u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_screen_faded_out() -> altv_wasm_shared::natives_result::ResultOf_is_screen_faded_out {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(469u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_cam_spline_node_index(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_cam_spline_node_index {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(470u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_cam_relative_heading(
            heading_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_cam_relative_heading {
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
                        &super::__shared::NumAsU64Arr::into_bytes(heading_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(471u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_create_cam_with_params(
            camName_: Option<&String>,
            posX_: f32,
            posY_: f32,
            posZ_: f32,
            rotX_: f32,
            rotY_: f32,
            rotZ_: f32,
            fov_: f32,
            p8_: bool,
            p9_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_create_cam_with_params {
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
                            super::__internal::send_to_host(&camName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(posX_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(posY_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(posZ_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotX_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotY_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotZ_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(fov_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p8_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p9_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(472u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_cam_spline_phase(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_cam_spline_phase {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(473u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_cam_far_clip(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_cam_far_clip {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(474u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_cam_coord(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_cam_coord {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(475u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_cinematic_cam_shaking() -> altv_wasm_shared::natives_result::ResultOf_is_cinematic_cam_shaking {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(476u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_first_person_aim_cam_relative_pitch_limits_this_update(
            p0_: f32,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_first_person_aim_cam_relative_pitch_limits_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(477u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_cam_shaking(
            cam_: i32,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_cam_shaking {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(478u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_code_gameplay_hint_active() -> altv_wasm_shared::natives_result::ResultOf_is_code_gameplay_hint_active {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(479u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_params(
            cam_: i32,
            posX_: f32,
            posY_: f32,
            posZ_: f32,
            rotX_: f32,
            rotY_: f32,
            rotZ_: f32,
            fieldOfView_: f32,
            p8_: i32,
            p9_: i32,
            p10_: i32,
            p11_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_params {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(posX_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(posY_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(posZ_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotX_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotY_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(rotZ_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(fieldOfView_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p8_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p9_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p10_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p11_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(480u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_cam_near_dof(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_cam_near_dof {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(481u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_animated_shake_script_global(
            p0_: Option<&String>,
            p1_: Option<&String>,
            p2_: Option<&String>,
            p3_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_animated_shake_script_global {
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
                            super::__internal::send_to_host(&p0_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p1_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&p2_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(482u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_cam_fov(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_cam_fov {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(483u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_dof_max_near_in_focus_distance(
            camera_: i32,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_dof_max_near_in_focus_distance {
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
                        &super::__shared::NumAsU64Arr::into_bytes(camera_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(484u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_create_cam(
            camName_: Option<&String>,
            p1_: bool,
        ) -> altv_wasm_shared::natives_result::ResultOf_create_cam {
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
                            super::__internal::send_to_host(&camName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(485u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_get_cam_near_clip(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_get_cam_near_clip {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(486u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_dof_focus_distance_bias(
            camera_: i32,
            p1_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_dof_focus_distance_bias {
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
                        &super::__shared::NumAsU64Arr::into_bytes(camera_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(487u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_follow_ped_cam_active() -> altv_wasm_shared::natives_result::ResultOf_is_follow_ped_cam_active {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(488u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cinematic_cam_shake_amplitude(
            p0_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cinematic_cam_shake_amplitude {
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
                        &super::__shared::NumAsU64Arr::into_bytes(p0_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(489u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_cam_near_clip(
            cam_: i32,
            nearClip_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_cam_near_clip {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(nearClip_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(490u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_stop_rendering_script_cams_using_catch_up(
            render_: bool,
            p1_: f32,
            p2_: i32,
            p3_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_stop_rendering_script_cams_using_catch_up {
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
                        &super::__shared::NumAsU64Arr::into_bytes(render_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p1_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p2_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(p3_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(491u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_follow_ped_cam_ladder_align_this_update() -> altv_wasm_shared::natives_result::ResultOf_set_follow_ped_cam_ladder_align_this_update {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(492u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_fly_cam_vertical_controls_this_update(
            cam_: i32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_fly_cam_vertical_controls_this_update {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(493u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_cam_playing_anim(
            cam_: i32,
            animName_: Option<&String>,
            animDictionary_: Option<&String>,
        ) -> altv_wasm_shared::natives_result::ResultOf_is_cam_playing_anim {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&animName_),
                        ),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(
                            super::__internal::send_to_host(&animDictionary_),
                        ),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(494u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_script_global_shaking() -> altv_wasm_shared::natives_result::ResultOf_is_script_global_shaking {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(495u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_fly_cam_coord_and_constrain(
            cam_: i32,
            x_: f32,
            y_: f32,
            z_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_fly_cam_coord_and_constrain {
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
                        &super::__shared::NumAsU64Arr::into_bytes(cam_ as i32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(x_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(y_ as f32),
                    );
                big_call_args
                    .extend_from_slice(
                        &super::__shared::NumAsU64Arr::into_bytes(z_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(496u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_set_gameplay_hint_camera_relative_vertical_offset(
            yOffset_: f32,
        ) -> altv_wasm_shared::natives_result::ResultOf_set_gameplay_hint_camera_relative_vertical_offset {
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
                        &super::__shared::NumAsU64Arr::into_bytes(yOffset_ as f32),
                    );
                std::mem::forget(big_call_args);
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(497u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_cinematic_idle_cam_rendering() -> altv_wasm_shared::natives_result::ResultOf_is_cinematic_idle_cam_rendering {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(498u32) };
                super::__internal::read_from_host(call_return)
            }
        }
        pub fn native_is_follow_vehicle_cam_active() -> altv_wasm_shared::natives_result::ResultOf_is_follow_vehicle_cam_active {
            #[allow(clippy::unnecessary_cast)]
            {
                #[allow(unused_variables, clippy::let_unit_value)]
                let call_return = unsafe { __custom_imports_native(499u32) };
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
