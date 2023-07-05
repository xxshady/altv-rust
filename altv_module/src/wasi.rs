use wasmer::{Instance, Module, Store, Function};
use wasmer_wasix::WasiEnv;

pub(crate) fn start(wasm_bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = Store::default();

    let module = Module::new(&store, wasm_bytes)?;

    let mut wasi_env = WasiEnv::builder("altv_rust_resource").finalize(&mut store)?;

    let mut import_object = wasi_env.import_object(&mut store, &module)?;

    fn altv_log(message: i32) {
        altv_sdk::helpers::log(message)
    }

    import_object.define("env", "altv_log", Function::new(&mut store, ));

    let instance = Instance::new(&mut store, &module, &import_object)?;

    wasi_env.initialize(&mut store, instance.clone())?;

    println!("Call WASI `_start` function...");
    // And we just call the `_start` function!
    let start = instance.exports.get_function("_start")?;
    start.call(&mut store, &[])?;

    Ok(())
}
