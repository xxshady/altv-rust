use indoc::formatdoc;
use std::{cmp::Ordering, fs::File, io::Write};

use crate::{
    helpers::{internal_name_of, native_type_to_rust, ValuePos, NATIVE_RETURN_IDENT},
    parser::{Native, NativeType, Param},
    result_struct_generator::result_struct_name_of,
};

pub(crate) fn gen(natives: &[Native]) -> File {
    let mut file = File::create("../wasm_host_natives/src/lib.rs").unwrap();

    let call_impls = gen_call_impls(natives);

    file.write_all(
        formatdoc! {"
        #![allow(unused_mut)]
        use std::cell::RefCell;
        use altv_wasm_shared::natives_result::*;
        use altv_sdk::ffi as sdk;
        use autocxx::prelude::*;
        use memory_buffer::MemoryBufferManager;

        mod memory_buffer;

        pub struct WasmNatives {{
            pub memory_buffers: RefCell<MemoryBufferManager>,
        }}

        impl WasmNatives {{
            pub fn new() -> Self {{
                Self {{
                    memory_buffers: RefCell::new(MemoryBufferManager::new()),
                }}
            }}
        }}

        impl wasm_host::gen::imports::extra_interfaces::WasmNatives for WasmNatives {{
            {call_impls}
        }}
    "}
        .as_bytes(),
    )
    .unwrap();

    file
}

fn gen_call_impls(natives: &[Native]) -> String {
    natives
        .iter()
        .map(|n| gen_one(n))
        .collect::<Vec<_>>()
        .join("\n    ")
}

fn gen_one(native: &Native) -> String {
    let Native {
        params,
        results,
        name,
        hash: _,
    } = native;

    let mut param_declarations = vec![];
    let mut param_serializations = vec![];
    let mut param_deserializations = vec![];
    let mut param_calls = vec![];
    let mut result_param_names = vec![];

    let mut params = params.to_vec();

    let void_return = match results[0].clone() {
        NativeType::Void => true,
        r#type => {
            params.insert(
                0,
                Param {
                    r#type,
                    internal_name: NATIVE_RETURN_IDENT.to_string(),
                    rust_name: NATIVE_RETURN_IDENT.to_string(),
                    r#ref: true,
                },
            );

            false
        }
    };

    for p in params.iter() {
        let native_type = p.r#type.clone();
        let cpp_value = cpp_value_of(native_type.clone(), &p.internal_name, p.r#ref);

        param_serializations.push({
            let ser = if p.r#ref {
                cpp_value.ser_ref
            } else {
                cpp_value.ser
            };

            format!("let mut {} = {ser};", p.internal_name)
        });

        if p.r#ref {
            param_deserializations.push(format!(
                "let {name} = {des};",
                name = p.internal_name,
                des = cpp_value.des
            ))
        }

        if p.internal_name != NATIVE_RETURN_IDENT {
            param_declarations.push(format!(
                "{}: {}",
                p.internal_name,
                native_type_to_rust(native_type, ValuePos::HostParam, p.r#ref)
            ));

            if p.r#ref {
                result_param_names.push(p.internal_name.clone());
            }
        }
    }

    params.sort_by(|a, b| match (a.r#ref, b.r#ref) {
        (true, true) => Ordering::Equal,
        (false, false) => Ordering::Equal,
        (true, false) => Ordering::Less,
        (false, true) => Ordering::Greater,
    });

    for p in params {
        let native_type = p.r#type.clone();
        let cpp_value = cpp_value_of(native_type.clone(), &p.internal_name, p.r#ref);

        param_calls.push({
            let call = if p.r#ref {
                cpp_value.call_ref
            } else {
                cpp_value.call
            };

            format!("{call},")
        });
    }

    let param_declarations = param_declarations.join(",\n");
    let param_serializations = param_serializations.join("\n");
    let param_deserializations = param_deserializations.join("\n");

    let param_calls = param_calls.join("\n");
    let result_param_names = result_param_names.join(",\n");

    let internal_name = internal_name_of(name);
    let result_struct_name = result_struct_name_of(&name);

    let ret = if void_return {
        "()".to_string()
    } else {
        NATIVE_RETURN_IDENT.to_string()
    };

    formatdoc! {"
        fn {internal_name}(&self, {param_declarations}) -> {result_struct_name} {{
            unsafe {{
                {param_serializations}
                let success = sdk::natives::{name}(
                    {param_calls}
                );
                {param_deserializations}
                
                {result_struct_name} {{
                    success,
                    ret: {ret},
                    {result_param_names}
                }}
            }}
        }}
    "}
}

struct CppValue {
    ser: String,
    ser_ref: String,
    des: String,
    call: String,
    call_ref: String,
}

impl CppValue {
    fn primitive(param_name: &str) -> CppValue {
        CppValue {
            ser: param_name.to_string(),
            ser_ref: if param_name == NATIVE_RETURN_IDENT { "Default::default()".to_string() } else { param_name.to_string() },
            des: param_name.to_string(),
            call: param_name.to_string(),
            call_ref: format!("&mut {param_name}"),
        }
    }
}

fn cpp_value_of(native_type: NativeType, param_name: &str, r#ref: bool) -> CppValue {
    match native_type {
        // i32
        NativeType::I32
        | NativeType::Interior
        | NativeType::Cam
        | NativeType::FireId
        | NativeType::Blip
        | NativeType::CarGenerator
        | NativeType::Group
        | NativeType::Weapon
        | NativeType::Texture
        | NativeType::TextureDict
        | NativeType::CoverPoint
        | NativeType::TaskSequence
        | NativeType::ColourIndex
        | NativeType::Sphere
        | NativeType::Pickup => CppValue::primitive(param_name),

        // u32
        NativeType::Hash
        | NativeType::Entity
        | NativeType::Object
        | NativeType::Ped
        | NativeType::Player
        | NativeType::ScrHandle
        | NativeType::Vehicle
        | NativeType::Train => CppValue::primitive(param_name),

        NativeType::F32 => CppValue::primitive(param_name),
        NativeType::String => CppValue {
            ser: format!("sdk_helpers::create_c_string_ptr({param_name})"),
            ser_ref: if param_name == NATIVE_RETURN_IDENT {
                "sdk::natives::create_null_c_string_ptr().within_unique_ptr()".to_string()
            } else {
                format!("sdk_helpers::create_c_string_ptr({param_name})")
            },
            des: format!("sdk_helpers::read_c_string_ptr({param_name})"),
            call: format!("{param_name}.as_ref().unwrap()"),
            call_ref: format!("{param_name}.pin_mut()"),
        },
        NativeType::Vector3 => CppValue {
            ser: format!("sdk::create_vector3().within_unique_ptr()"),
            ser_ref: if param_name == NATIVE_RETURN_IDENT {
                format!("sdk::create_vector3().within_unique_ptr()")
            } else {
                format!("sdk_helpers::create_vector3_ptr({param_name})")
            },
            des: format!("sdk_helpers::read_cpp_vector3({param_name})"),
            call: format!("{param_name}.as_ref().unwrap()"),
            call_ref: format!("{param_name}.pin_mut()"),
        },
        NativeType::Void => unreachable!(),
        NativeType::Boolean => CppValue::primitive(param_name),

        // Any ref is used for memory buffers
        NativeType::MemoryBuffer => unreachable!(),
        NativeType::Any => {
            match (r#ref, param_name) {
                (true, NATIVE_RETURN_IDENT) => {
                    CppValue::primitive(param_name)
                },
                (true, _) => {
                    CppValue {
                        ser_ref: format!(
                            "self.memory_buffers.borrow_mut().get_mut_ptr({param_name}) as *mut c_void"
                        ),
                        call_ref: param_name.to_string(),
    
                        des: format!("0"), // does not matter anyway
                        ser: format!("MEMORY BUFFER IS ALWAYS REF"),
                        call: format!("MEMORY BUFFER IS ALWAYS REF"),
                    }
                },
                (false, _) => {
                    CppValue::primitive(param_name)
                },
            }
        }
    }
}
