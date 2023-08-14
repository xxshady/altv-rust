use wasmtime::*;
use wasmtime_wasi::{WasiCtxBuilder, WasiCtx};
use std::{io::BufReader, error::Error, fmt::Debug};

use crate::resource_manager::ResourceController;

wasm_bindgen::host!("../wasm.interface");

struct State<'a> {
    resource: &'a ResourceController,
    wasi: WasiCtx,
}

impl<'a> host::imports::Imports for State<'a> {
    fn log_error(&self, message: String) {
        unsafe { altv_sdk::helpers::log_error_with_resource(&message, self.resource.ptr) }
    }

    fn log(&self, message: String) {
        unsafe { altv_sdk::helpers::log_with_resource(&message, self.resource.ptr) }
    }
}

pub(crate) fn start(wasm_bytes: &[u8], resource: &ResourceController) -> wasmtime::Result<()> {
    let engine = Engine::default();
    let mut linker = Linker::<State>::new(&engine);

    let module = Module::from_binary(&engine, wasm_bytes)?;

    let wasi = WasiCtxBuilder::new().build();
    let mut store = Store::new(&engine, State { wasi, resource });

    wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi)?;
    host::imports::add_to_linker(&mut linker);

    let instance = linker.instantiate(&mut store, &module)?;

    let mut exports = host::exports::CustomExports::new(store, instance);

    let result = exports.call_main();
    logger::debug!("call main result: {result:?}");

    Ok(())
}
