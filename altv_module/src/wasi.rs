use std::{io::BufReader, error::Error, fmt::Debug};

use wasmer::{Instance, Module, Store};
use wasmer_wasix::{WasiEnv, Pipe};

use crate::types::StdoutReader;

// wai_bindgen_wasmer::export!("../wasmer_test/api.wai");
// wai_bindgen_wasmer::import!("../wasmer_test/exports.wai");

// impl Debug for exports::Exports {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "Exports")
//     }
// }

// struct ResourceApi;

// impl api::Api for ResourceApi {
//     fn create_marker(&mut self, marker_type: u32, x: f32, y: f32, z: f32) -> Option<u32> {
//         use altv_sdk::ffi as sdk;
//         let marker = unsafe {
//             sdk::ICore::CreateMarker(
//                 marker_type,
//                 x,
//                 y,
//                 z,
//                 255,
//                 255,
//                 255,
//                 255,
//                 false,
//                 0,
//                 std::ptr::null_mut(),
//             )
//         };
//         if marker.is_null() {
//             return None;
//         }

//         let base_object = unsafe { sdk::marker::to_base_object(marker) };
//         if base_object.is_null() {
//             return None;
//         }

//         Some(unsafe { sdk::IBaseObject::GetID(base_object) })
//     }

//     fn log_error(&mut self, str: &str) {
//         // TODO: get resource ptr somehow
//         unsafe {
//             altv_sdk::helpers::log_error_with_resource(str, std::ptr::null_mut());
//         }
//     }
// }

pub(crate) fn start(
    wasm_bytes: &[u8],
) -> Result<(Instance, StdoutReader, (), Store), Box<dyn Error>> {
    todo!();

    let mut store = Store::default();

    let module = Module::new(&store, wasm_bytes)?;

    let (stdout_sender, stdout_reader) = Pipe::channel();
    let stdout_reader = BufReader::new(stdout_reader);

    let mut wasi_env = WasiEnv::builder("altv_rust_resource")
        .stdout(Box::new(stdout_sender))
        .finalize(&mut store)?;

    let mut import_object = wasi_env.import_object(&mut store, &module)?;

    // let initializer = api::add_to_imports(&mut store, &mut import_object, ResourceApi);

    // let (exports, instance) =
    //     exports::Exports::instantiate(&mut store, &module, &mut import_object)?;

    // initializer(&instance, &store)?;

    // wasi_env.initialize(&mut store, instance.clone())?;

    // logger::debug!("calling wasi _start");
    // let start = instance.exports.get_function("_start")?;
    // start.call(&mut store, &[])?;

    // Ok((instance, stdout_reader, exports, store))
}
