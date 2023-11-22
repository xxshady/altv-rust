use indoc::formatdoc;
use std::fs::{self, File};

use crate::{
    helpers::{native_type_to_rust, ValuePos},
    parser::Native,
};

pub(crate) fn prepare() -> fs::File {
    File::create("../altv_wasm_shared/src/natives_result.rs").unwrap()
}

pub(crate) fn gen(native: &Native) -> String {
    let Native {
        params,
        name,
        results,
        hash: _,
    } = native;

    let ret_declaration = format!(
        "pub ret: {},",
        native_type_to_rust(results[0].clone(), ValuePos::GuestResult, false)
    );

    let ref_param_declarations: String = params
        .iter()
        .filter_map(|p| {
            if !p.r#ref {
                return None;
            }
            Some(format!(
                "pub {}: {}",
                p.name,
                native_type_to_rust(p.r#type.clone(), ValuePos::GuestResult, p.r#ref),
            ))
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let struct_name = result_struct_name_of(name);

    formatdoc! {"
        #[derive(serde::Serialize, serde::Deserialize)]
        pub struct {struct_name} {{
            pub success: bool,
            {ret_declaration}
            {ref_param_declarations}
        }}
    "}
}

pub(crate) fn result_struct_name_of(native_name: &str) -> String {
    format!("ResultOf_{native_name}")
}
