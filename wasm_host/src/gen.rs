
    // AUTO-GENERATED
    // All manual changes will be overwritten

mod host {
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
    pub type FreeFunc = wasmtime::TypedFunc<FatPtr, ()>;
    pub type AllocFunc = wasmtime::TypedFunc<Size, Ptr>;
    pub mod exports {
        pub struct Exports<S> {
            prop_on_tick: wasmtime::TypedFunc<(), ()>,
            prop_on_base_object_create: wasmtime::TypedFunc<(u64, u32), ()>,
            prop_on_base_object_destroy: wasmtime::TypedFunc<(u64, u32), ()>,
            memory: wasmtime::Memory,
            store: wasmtime::Store<S>,
            alloc: super::AllocFunc,
            free: super::FreeFunc,
            pre_main: wasmtime::TypedFunc<(), ()>,
            main: wasmtime::TypedFunc<(), ()>,
        }
        impl<S> Exports<S> {
            pub fn new(
                mutate_big_call_ptr: impl FnOnce(&mut S) -> &mut super::Ptr,
                mut store: wasmtime::Store<S>,
                instance: wasmtime::Instance,
            ) -> Self {
                let mut exports = Self {
                    prop_on_tick: instance
                        .get_typed_func(&mut store, stringify!(__custom_exports_on_tick))
                        .unwrap(),
                    prop_on_base_object_create: instance
                        .get_typed_func(
                            &mut store,
                            stringify!(__custom_exports_on_base_object_create),
                        )
                        .unwrap(),
                    prop_on_base_object_destroy: instance
                        .get_typed_func(
                            &mut store,
                            stringify!(__custom_exports_on_base_object_destroy),
                        )
                        .unwrap(),
                    memory: instance.get_memory(&mut store, "memory").unwrap(),
                    alloc: instance
                        .get_typed_func(&mut store, "__custom_alloc")
                        .unwrap(),
                    free: instance.get_typed_func(&mut store, "__custom_free").unwrap(),
                    pre_main: instance.get_typed_func(&mut store, "__pre_main").unwrap(),
                    main: instance.get_typed_func(&mut store, "main").unwrap(),
                    store,
                };
                {
                    let (ptr, _size) = exports
                        .alloc_bytes(
                            &[1_u8; super::__shared::BYTES_TO_STORE_U64_32_TIMES],
                        )
                        .unwrap();
                    *mutate_big_call_ptr(exports.store.data_mut()) = ptr;
                    let init_big_call: wasmtime::TypedFunc<
                        (super::__shared::Ptr,),
                        (),
                    > = instance
                        .get_typed_func(&mut exports.store, "__init_big_call")
                        .unwrap();
                    init_big_call.call(&mut exports.store, (ptr,)).unwrap();
                }
                exports
            }
            pub fn store_mut(&mut self) -> &mut wasmtime::Store<S> {
                &mut self.store
            }
            pub fn alloc_bytes(
                &mut self,
                bytes: &[u8],
            ) -> wasmtime::Result<(super::__shared::Ptr, super::__shared::Size)> {
                let size = bytes.len().try_into()?;
                let ptr = self.alloc.call(&mut self.store, size)?;
                self.memory.write(&mut self.store, ptr as usize, bytes)?;
                Ok((ptr, size))
            }
            fn clone_bytes_to_guest(
                &mut self,
                bytes: &[u8],
            ) -> wasmtime::Result<super::__shared::FatPtr> {
                let (ptr, size) = self.alloc_bytes(bytes)?;
                Ok(super::__shared::to_fat_ptr(ptr, size))
            }
            fn send_to_guest<T: ?Sized + serde::Serialize>(
                &mut self,
                data: &T,
            ) -> wasmtime::Result<super::__shared::FatPtr> {
                let encoded: Vec<u8> = bincode::serialize(&data)?;
                self.clone_bytes_to_guest(&encoded)
            }
            fn send_str_to_guest(
                &mut self,
                str: &str,
            ) -> wasmtime::Result<super::__shared::FatPtr> {
                self.clone_bytes_to_guest(str.as_bytes())
            }
            fn read_to_buffer(
                &mut self,
                fat_ptr: super::__shared::FatPtr,
            ) -> wasmtime::Result<Vec<u8>> {
                let (ptr, size) = super::__shared::from_fat_ptr(fat_ptr);
                let mut buffer = vec![0; size as usize];
                self.memory.read(&self.store, ptr as usize, &mut buffer)?;
                self.free.call(&mut self.store, fat_ptr).unwrap();
                Ok(buffer)
            }
            fn read_from_guest<T: serde::de::DeserializeOwned>(
                &mut self,
                fat_ptr: super::__shared::FatPtr,
            ) -> wasmtime::Result<T> {
                let buffer = self.read_to_buffer(fat_ptr)?;
                Ok(bincode::deserialize(&buffer)?)
            }
            fn read_string_from_guest(
                &mut self,
                fat_ptr: super::__shared::FatPtr,
            ) -> wasmtime::Result<String> {
                let buffer = self.read_to_buffer(fat_ptr)?;
                Ok(String::from_utf8(buffer)?)
            }
            pub fn call_pre_main(&mut self) -> wasmtime::Result<()> {
                self.pre_main.call(&mut self.store, ())
            }
            pub fn call_main(&mut self) -> wasmtime::Result<()> {
                self.main.call(&mut self.store, ())
            }
            pub fn call_on_tick(&mut self) -> wasmtime::Result<()> {
                #[allow(clippy::unnecessary_cast)]
                {
                    #[allow(unused_variables, clippy::let_unit_value)]
                    let call_return = self.prop_on_tick.call(&mut self.store, ())?;
                    Ok(())
                }
            }
            pub fn call_on_base_object_create(
                &mut self,
                ptr: altv_wasm_shared::BaseObjectPtr,
                ty: altv_wasm_shared::BaseObjectTypeRaw,
            ) -> wasmtime::Result<()> {
                #[allow(clippy::unnecessary_cast)]
                {
                    let ptr = ptr as u64;
                    let ty = ty as u32;
                    #[allow(unused_variables, clippy::let_unit_value)]
                    let call_return = self
                        .prop_on_base_object_create
                        .call(&mut self.store, (ptr, ty))?;
                    Ok(())
                }
            }
            pub fn call_on_base_object_destroy(
                &mut self,
                ptr: altv_wasm_shared::BaseObjectPtr,
                ty: altv_wasm_shared::BaseObjectTypeRaw,
            ) -> wasmtime::Result<()> {
                #[allow(clippy::unnecessary_cast)]
                {
                    let ptr = ptr as u64;
                    let ty = ty as u32;
                    #[allow(unused_variables, clippy::let_unit_value)]
                    let call_return = self
                        .prop_on_base_object_destroy
                        .call(&mut self.store, (ptr, ty))?;
                    Ok(())
                }
            }
        }
    }
    const _: &str = include_str!(
        r#"C:\\dev\\rust\\altv-rust\\wasm_host\\../wasm.interface"#
    );
    pub mod imports {
        pub trait Imports {
            type ExtraInterfaceWasmNatives: extra_interfaces::WasmNatives;
            fn get_memory(&self) -> Option<wasmtime::Memory>;
            fn set_memory(&mut self, memory: wasmtime::Memory);
            fn get_free(&self) -> Option<super::FreeFunc>;
            fn set_free(&mut self, free: super::FreeFunc);
            fn get_alloc(&self) -> Option<super::AllocFunc>;
            fn set_alloc(&mut self, alloc: super::AllocFunc);
            fn get_big_call_ptr(&self) -> super::Ptr;
            fn get_wasm_natives(&self) -> &Self::ExtraInterfaceWasmNatives;
            fn log(&self, message: String);
            fn log_error(&self, message: String);
            fn destroy_base_object(&self, ptr: altv_wasm_shared::BaseObjectPtr);
            fn base_object_get_id(&self, ptr: altv_wasm_shared::BaseObjectPtr) -> u32;
            fn base_object_get_remote_id(
                &self,
                ptr: altv_wasm_shared::BaseObjectPtr,
            ) -> u32;
            fn world_object_get_pos(
                &self,
                ptr: altv_wasm_shared::BaseObjectPtr,
            ) -> shared::Vector3;
            fn world_object_set_pos(
                &self,
                ptr: altv_wasm_shared::BaseObjectPtr,
                value: shared::Vector3,
            );
            fn world_object_get_dimension(
                &self,
                ptr: altv_wasm_shared::BaseObjectPtr,
            ) -> i32;
            fn world_object_set_dimension(
                &self,
                ptr: altv_wasm_shared::BaseObjectPtr,
                value: i32,
            );
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
            ) -> altv_wasm_shared::BaseObjectPtr;
            fn vehicle_set_fuel_level(
                &self,
                ptr: altv_wasm_shared::BaseObjectPtr,
                value: f32,
            );
            fn vehicle_get_fuel_level(
                &self,
                ptr: altv_wasm_shared::BaseObjectPtr,
            ) -> f32;
            fn alloc_memory_buffer(&self, size: u16) -> shared::MemoryBufferId;
            fn dealloc_memory_buffer(&self, id: shared::MemoryBufferId);
            fn read_memory_buffer(&self, id: shared::MemoryBufferId) -> Vec<u8>;
        }
        pub mod extra_interfaces {
            pub trait WasmNatives: Sized {
                fn native_get_dlc_weapon_data(
                    &self,
                    dlc_weapon_index_: i32,
                    out_data_: shared::MemoryBufferId,
                ) -> altv_wasm_shared::natives_result::ResultOfGetDlcWeaponData;
            }
        }
        pub fn add_to_linker<U: Imports>(linker: &mut wasmtime::Linker<U>) {
            use extra_interfaces::*;
            fn get_memory<U: Imports>(
                caller: &mut wasmtime::Caller<U>,
            ) -> wasmtime::Memory {
                let Some(wasmtime::Extern::Memory(memory)) = caller.get_export("memory")
                else { panic!("Failed to get memory export") };
                memory
            }
            fn read_big_call_args<U: Imports>(
                caller: &mut wasmtime::Caller<U>,
            ) -> &'static std::thread::LocalKey<std::cell::RefCell<Vec<u8>>> {
                thread_local! {
                    static ARGS : std::cell::RefCell < Vec < u8 >> =
                    std::cell::RefCell::new(vec![0u8;
                    super::__shared::BYTES_TO_STORE_U64_32_TIMES]);
                }
                let big_call_ptr = caller.data().get_big_call_ptr();
                ARGS.with_borrow_mut(|args| {
                    get_memory(caller)
                        .read(caller, big_call_ptr as usize, args)
                        .unwrap();
                });
                &ARGS
            }
            fn get_memory_and<
                U: Imports,
                Params: wasmtime::WasmParams,
                Results: wasmtime::WasmResults,
            >(
                caller: &mut wasmtime::Caller<U>,
                and: &'static str,
            ) -> (wasmtime::Memory, wasmtime::TypedFunc<Params, Results>) {
                let memory = get_memory(caller);
                let Some(wasmtime::Extern::Func(func)) = caller.get_export(and) else {
                    panic!("Failed to get {and:?} export")
                };
                (memory, func.typed::<Params, Results>(caller).unwrap())
            }
            fn read_to_buffer<U: Imports>(
                mut caller: &mut wasmtime::Caller<U>,
                fat_ptr: super::__shared::FatPtr,
                call_free: bool,
            ) -> wasmtime::Result<Vec<u8>> {
                let memory = caller.data().get_memory();
                let free = caller.data().get_free();
                let (memory, free) = if free.is_some() {
                    (memory.unwrap(), free.unwrap())
                } else {
                    get_memory_and(caller, "__custom_free")
                };
                let (ptr, size) = super::__shared::from_fat_ptr(fat_ptr);
                let mut buffer = vec![0; size as usize];
                memory.read(&caller, ptr as usize, &mut buffer)?;
                if call_free {
                    free.call(&mut caller, fat_ptr)?;
                }
                let data = caller.data_mut();
                data.set_memory(memory);
                data.set_free(free);
                Ok(buffer)
            }
            fn read_from_guest<U: Imports, T: serde::de::DeserializeOwned>(
                caller: &mut wasmtime::Caller<U>,
                fat_ptr: super::__shared::FatPtr,
            ) -> wasmtime::Result<T> {
                let buffer = read_to_buffer(caller, fat_ptr, true)?;
                Ok(bincode::deserialize(&buffer)?)
            }
            fn read_string_ref_from_guest<U: Imports>(
                caller: &mut wasmtime::Caller<U>,
                fat_ptr: super::__shared::FatPtr,
            ) -> wasmtime::Result<String> {
                let buffer = read_to_buffer(caller, fat_ptr, false)?;
                Ok(String::from_utf8(buffer)?)
            }
            fn clone_bytes_to_guest<U: Imports>(
                mut caller: &mut wasmtime::Caller<U>,
                bytes: &[u8],
            ) -> wasmtime::Result<super::__shared::FatPtr> {
                let (memory, alloc) = {
                    let data = caller.data();
                    (data.get_memory(), data.get_alloc())
                };
                let (memory, alloc) = if alloc.is_some() {
                    (memory.unwrap(), alloc.unwrap())
                } else {
                    get_memory_and(caller, "__custom_alloc")
                };
                let size = bytes.len().try_into()?;
                let ptr = alloc.call(&mut caller, size)?;
                memory.write(&mut caller, ptr as usize, bytes)?;
                let data = caller.data_mut();
                data.set_memory(memory);
                data.set_alloc(alloc);
                Ok(super::__shared::to_fat_ptr(ptr, size))
            }
            fn send_to_guest<U: Imports, T: ?Sized + serde::Serialize>(
                caller: &mut wasmtime::Caller<U>,
                data: &T,
            ) -> wasmtime::Result<super::__shared::FatPtr> {
                let bytes = bincode::serialize(&data)?;
                clone_bytes_to_guest(caller, &bytes)
            }
            fn send_string_to_guest<U: Imports>(
                caller: &mut wasmtime::Caller<U>,
                string: String,
            ) -> wasmtime::Result<super::__shared::FatPtr> {
                clone_bytes_to_guest(caller, string.as_bytes())
            }
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(log),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, message: super::__shared::FatPtr| {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let message = read_string_ref_from_guest(
                                    &mut caller,
                                    message,
                                )
                                .unwrap();
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data().log(message);
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(log_error),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, message: super::__shared::FatPtr| {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let message = read_string_ref_from_guest(
                                    &mut caller,
                                    message,
                                )
                                .unwrap();
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data().log_error(message);
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(destroy_base_object),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, ptr: u64| {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let ptr = ptr as altv_wasm_shared::BaseObjectPtr;
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data().destroy_base_object(ptr);
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(base_object_get_id),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, ptr: u64| -> u32 {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let ptr = ptr as altv_wasm_shared::BaseObjectPtr;
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data().base_object_get_id(ptr);
                            call_return as u32
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(base_object_get_remote_id),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, ptr: u64| -> u32 {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let ptr = ptr as altv_wasm_shared::BaseObjectPtr;
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller
                                .data()
                                .base_object_get_remote_id(ptr);
                            call_return as u32
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(world_object_get_pos),
                    #[allow(unused_mut)]
                    |
                        mut caller: wasmtime::Caller<U>,
                        ptr: u64,
                    | -> super::__shared::FatPtr {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let ptr = ptr as altv_wasm_shared::BaseObjectPtr;
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data().world_object_get_pos(ptr);
                            send_to_guest(&mut caller, &call_return).unwrap()
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(world_object_set_pos),
                    #[allow(unused_mut)]
                    |
                        mut caller: wasmtime::Caller<U>,
                        ptr: u64,
                        value: super::__shared::FatPtr|
                    {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let ptr = ptr as altv_wasm_shared::BaseObjectPtr;
                            let value = read_from_guest(&mut caller, value).unwrap();
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller
                                .data()
                                .world_object_set_pos(ptr, value);
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(world_object_get_dimension),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, ptr: u64| -> i32 {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let ptr = ptr as altv_wasm_shared::BaseObjectPtr;
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller
                                .data()
                                .world_object_get_dimension(ptr);
                            call_return as i32
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(world_object_set_dimension),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, ptr: u64, value: i32| {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let ptr = ptr as altv_wasm_shared::BaseObjectPtr;
                            let value = value as i32;
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller
                                .data()
                                .world_object_set_dimension(ptr, value);
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(create_local_vehicle),
                    #[allow(unused_mut)]
                    |
                        mut caller: wasmtime::Caller<U>,
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
                    | -> u64 {
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
                            let use_streaming = use_streaming == 1;
                            let streaming_distance = streaming_distance as u32;
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller
                                .data()
                                .create_local_vehicle(
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
                                );
                            call_return as u64
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(vehicle_set_fuel_level),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, ptr: u64, value: f32| {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let ptr = ptr as altv_wasm_shared::BaseObjectPtr;
                            let value = value as f32;
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller
                                .data()
                                .vehicle_set_fuel_level(ptr, value);
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(vehicle_get_fuel_level),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, ptr: u64| -> f32 {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let ptr = ptr as altv_wasm_shared::BaseObjectPtr;
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data().vehicle_get_fuel_level(ptr);
                            call_return as f32
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(alloc_memory_buffer),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, size: u32| -> u32 {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let size = size as u16;
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data().alloc_memory_buffer(size);
                            call_return as u32
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(dealloc_memory_buffer),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, id: u32| {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let id = id as shared::MemoryBufferId;
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data().dealloc_memory_buffer(id);
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(read_memory_buffer),
                    #[allow(unused_mut)]
                    |
                        mut caller: wasmtime::Caller<U>,
                        id: u32,
                    | -> super::__shared::FatPtr {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            let id = id as shared::MemoryBufferId;
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data().read_memory_buffer(id);
                            send_to_guest(&mut caller, &call_return).unwrap()
                        }
                    },
                )
                .unwrap();
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(native),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, func_index: u32| -> u64 {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            match func_index {
                                0u32 => {
                                    let (dlc_weapon_index_, out_data_) = read_big_call_args(
                                            &mut caller,
                                        )
                                        .with_borrow(|big_call_args| {
                                            (
                                                <i32 as super::__shared::NumAsU64Arr>::from_bytes(
                                                    big_call_args[0usize..8usize].try_into().unwrap(),
                                                ) as i32,
                                                <u32 as super::__shared::NumAsU64Arr>::from_bytes(
                                                    big_call_args[8usize..16usize].try_into().unwrap(),
                                                ) as shared::MemoryBufferId,
                                            )
                                        });
                                    #[allow(unused_variables, clippy::let_unit_value)]
                                    let call_return = caller
                                        .data()
                                        .get_wasm_natives()
                                        .native_get_dlc_weapon_data(dlc_weapon_index_, out_data_);
                                    send_to_guest(&mut caller, &call_return).unwrap()
                                }
                                _ => {
                                    panic!(
                                        "Unknown multi func index: {func_index} in func: {}",
                                        stringify!(native)
                                    );
                                }
                            }
                        }
                    },
                )
                .unwrap();
        }
    }
    const _: &str = include_str!(
        r#"C:\\dev\\rust\\altv-rust\\wasm_host\\../wasm.interface"#
    );
    const _: &str = include_str!(
        r#"C:\\dev\\rust\\altv-rust\\wasm_host\\../wasm_natives.interface"#
    );
}
pub use host::*;
