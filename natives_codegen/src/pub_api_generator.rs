use indoc::formatdoc;
use std::fs::{self, File};

use crate::{
    helpers::{internal_name_of, native_type_to_rust, ValuePos},
    parser::Native,
    result_struct_generator::result_struct_name_of,
};

pub(crate) fn prepare() -> fs::File {
    File::create("../altv_wasm/src/natives.rs").unwrap()
}

pub(crate) fn gen(native: &Native) -> String {
    let Native {
        params,
        name,
        results: _,
        hash: _,
    } = native;

    let param_declarations: String = params
        .iter()
        .map(|p| {
            format!(
                "{}: {}",
                p.rust_name,
                native_type_to_rust(p.r#type.clone(), ValuePos::GuestParam, p.r#ref)
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let passed_params = params
        .iter()
        .map(|p| p.rust_name.clone())
        .collect::<Vec<_>>()
        .join(",\n");

    let internal_name = internal_name_of(name);
    let result_struct_name = result_struct_name_of(&name);

    formatdoc! {"
        pub fn {name}({param_declarations}) -> altv_wasm_shared::natives_result::{result_struct_name} {{
            crate::__imports::{internal_name}({passed_params})
        }}
    "}
}
