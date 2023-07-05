use std::io::{BufReader, BufRead};

use wasmer::{Instance, Module, Store, Function};
use wasmer_wasix::{WasiEnv, Pipe};

pub(crate) fn start(wasm_bytes: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut store = Store::default();

    let module = Module::new(&store, wasm_bytes)?;

    let (stdout_sender, stdout_reader) = Pipe::channel();
    let mut wasi_env = WasiEnv::builder("altv_rust_resource")
        .stdout(Box::new(stdout_sender))
        .finalize(&mut store)?;

    let mut buf_reader = BufReader::new(stdout_reader);

    let import_object = wasi_env.import_object(&mut store, &module)?;

    let instance = Instance::new(&mut store, &module, &import_object)?;

    wasi_env.initialize(&mut store, instance.clone())?;

    logger::debug!("calling wasi _start");
    let start = instance.exports.get_function("_start")?;
    start.call(&mut store, &[])?;

    let mut stdout_buf = String::new();
    while buf_reader.read_line(&mut stdout_buf).is_ok() {
        logger::debug!("buf data: {stdout_buf}");
        stdout_buf.clear();
    }

    Ok(())
}
