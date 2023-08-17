use wasmtime::*;
use wasmtime_wasi::{WasiCtxBuilder, WasiCtx};
use altv_sdk::ffi as sdk;

wasm_codegen::host!("../wasm.interface");
pub type Exports = host::exports::Exports<State>;

impl std::fmt::Debug for Exports {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Exports {{}}")
    }
}

pub struct State {
    // TODO: some safe wrapper over this unsafe shit
    resource_ptr: *mut sdk::shared::AltResource,
    wasi: WasiCtx,
}

impl host::imports::Imports for State {
    fn log_error(&self, message: String) {
        unsafe { altv_sdk::helpers::log_error_with_resource(&message, self.resource_ptr) }
    }

    fn log(&self, message: String) {
        unsafe { altv_sdk::helpers::log_with_resource(&message, self.resource_ptr) }
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
