use indoc::formatdoc;
use std::fs::{self, File};

use crate::{
    helpers::{internal_name_of, native_type_to_rust, ValuePos},
    parser::Native,
    result_struct_generator::result_struct_name_of,
};

pub(crate) fn prepare() -> fs::File {
    File::create("out/pub_api.rs").unwrap()
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
                p.name,
                native_type_to_rust(p.r#type.clone(), ValuePos::GuestParam)
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let passed_params = params
        .iter()
        .map(|p| p.name.clone())
        .collect::<Vec<_>>()
        .join(",\n");

    let internal_name = internal_name_of(name);
    let result_struct_name = result_struct_name_of(&name);

    formatdoc! {"
        fn {name}({param_declarations}) -> {result_struct_name} {{
            {internal_name}({passed_params})
        }}
    "}
}
