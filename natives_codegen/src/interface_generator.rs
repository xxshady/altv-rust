use indoc::formatdoc;
use std::{
    fs::{self, File},
    io::Write,
};

use crate::{
    parser::{Native, NativeType},
    result_struct_generator::result_struct_name_of,
};

pub(crate) fn gen(natives: &[Native]) {
    let mut file = create_file();

    let types = gen_types(natives);
    let funcs = gen_funcs(natives);

    file.write_all(
        formatdoc! {"
        types {{
            {types}
        }}

        imports {{
            native[
                {funcs}
            ]
        }}

        exports {{
        }}
    "}
        .as_bytes(),
    )
    .unwrap();
}

fn create_file() -> fs::File {
    File::create("../wasm_natives.interface").unwrap()
}

fn gen_funcs(natives: &[Native]) -> String {
    natives
        .iter()
        .map(
            |Native {
                 params,
                 name,
                 results: _,
                 hash: _,
             }: &Native| {
                let param_declarations: String = params
                    .iter()
                    .map(|p| format!("{}: {}", &p.name, native_type_to_interface(p.r#type.clone(), p.r#ref)))
                    .collect::<Vec<_>>()
                    .join(", ");

                let struct_full_path = result_struct_full_path(&name);

                format!("{name}({param_declarations}) -> {struct_full_path}")
            },
        )
        .collect::<Vec<_>>()
        .join("\n        ")
}

fn gen_types(natives: &[Native]) -> String {
    natives
        .iter()
        .map(
            |Native {
                 name,
                 params: _,
                 results: _,
                 hash: _,
             }: &Native| {
                let struct_full_path = result_struct_full_path(name);

                format!("type: {struct_full_path} kind: FatPtr repr: FatPtr can_be_param: false")
            },
        )
        .collect::<Vec<_>>()
        .join("\n    ")
}

fn native_type_to_interface(native_type: NativeType, r#ref: bool) -> &'static str {
    match native_type {
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
        | NativeType::Pickup
        => "i32",

        NativeType::Hash
        | NativeType::Entity
        | NativeType::Object
        | NativeType::Ped

        | NativeType::Player
        | NativeType::ScrHandle
        | NativeType::Vehicle
        | NativeType::Train
         => "u32",

        NativeType::F32 => "f32",
        NativeType::String => "Option<&String>",
        NativeType::Vector3 => "Option<&shared::Vector3>",
        NativeType::Boolean => "bool",

        NativeType::Void => panic!("Void must not be used anywhere"),

        // Any ref is used for memory buffers
        NativeType::MemoryBuffer => unreachable!(),
        NativeType::Any => {
            if r#ref { "shared::MemoryBufferId" }
            else { "i32" }
        },
    }
}

pub(crate) fn result_struct_full_path(native_name: &str) -> String {
    let struct_name = result_struct_name_of(native_name);
    format!("altv_wasm_shared::natives_result::{struct_name}")
}
