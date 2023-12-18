use indoc::formatdoc;
use std::{
    fs::{self, File},
    io::Write,
};

use crate::{
    helpers::{internal_name_of, native_type_to_rust, ValuePos},
    parser::{Native, NativeType},
    result_struct_generator::result_struct_name_of,
};

pub(crate) fn prepare() -> fs::File {
    let mut file = File::create("../altv_wasm/src/natives.rs").unwrap();

    file.write_all(
        formatdoc! {"
            use crate::script_id::{{VehicleScriptId, AsEntityScriptId}};
        "}
        .as_bytes(),
    )
    .unwrap();

    file
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
                pub_api_types::native_type_to_pub_api(
                    p.r#type.clone(),
                    ValuePos::GuestParam,
                    p.r#ref
                )
                .r#type()
            )
        })
        .collect::<Vec<_>>()
        .join(",\n");

    let passed_params = params
        .iter()
        .map(|p| {
            pub_api_types::native_type_to_pub_api(p.r#type.clone(), ValuePos::GuestParam, p.r#ref)
                .usage(&p.rust_name)
        })
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

mod pub_api_types {
    use super::*;

    pub(super) trait PubApiType {
        fn r#type(&self) -> &'static str;
        fn usage(&self, param_name: &str) -> String;
    }

    pub(super) struct Vehicle {
        r#type: &'static str,
    }

    impl PubApiType for Vehicle {
        fn r#type(&self) -> &'static str {
            self.r#type
        }

        fn usage(&self, param_name: &str) -> String {
            format!("{param_name}.0")
        }
    }

    pub(super) struct Entity {
        r#type: &'static str,
    }

    impl PubApiType for Entity {
        fn r#type(&self) -> &'static str {
            self.r#type
        }

        fn usage(&self, param_name: &str) -> String {
            format!("{param_name}.as_entity_script_id()")
        }
    }

    pub(super) struct AnythingElse {
        r#type: &'static str,
    }

    impl PubApiType for AnythingElse {
        fn r#type(&self) -> &'static str {
            self.r#type
        }

        fn usage(&self, param_name: &str) -> String {
            param_name.to_string()
        }
    }

    pub(super) fn native_type_to_pub_api(
        native_type: NativeType,
        pos: ValuePos,
        r#ref: bool,
    ) -> Box<dyn PubApiType> {
        match native_type {
            NativeType::Vehicle => Box::new(Vehicle {
                r#type: "&VehicleScriptId",
            }),
            NativeType::Entity => Box::new(Entity {
                r#type: "impl AsEntityScriptId",
            }),
            _ => Box::new(AnythingElse {
                r#type: native_type_to_rust(native_type, pos, r#ref),
            }),
        }
    }
}
