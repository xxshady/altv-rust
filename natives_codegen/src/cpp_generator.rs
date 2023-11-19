use indoc::formatdoc;
use std::{
    fs::{self, File},
    io::Write,
};

use crate::{
    helpers::NATIVE_RETURN_IDENT,
    parser::{Native, NativeType, Param},
};

pub(crate) fn prepare() -> fs::File {
    let mut file = File::create("../altv_sdk/src/natives.h").unwrap();

    file.write_all(
        formatdoc! {"
            #pragma once
            #include \"alt_bridge.h\"

            namespace natives {{
                using Success = bool;

                std::shared_ptr<alt::INative::Context> ctx;

                void init() {{
                    ctx = alt::ICore::Instance().CreateNativesContext();
                }}

                static char* save_c_string(const char* str) {{
                    if (str == nullptr) return nullptr;
                    static char* stringValues[256] = {{ 0 }};
                    static int nextString = 0;
                    if (stringValues[nextString]) free(stringValues[nextString]);
                    char* _str = _strdup(str);
                    stringValues[nextString] = _str;
                    nextString = (nextString + 1) % 256;
                    return _str;
                }}

                char* clone_c_string(const char* str) {{
                    size_t stringSize = strlen(str);
                    char* writable = new char[stringSize + 1];
                    std::memcpy(writable, str, stringSize);
                    writable[stringSize] = '\\0';
                    return writable;
                }}


                class CStringPtr {{
                public:
                    char* ptr = nullptr;

                    CStringPtr(std::string content) : ptr(save_c_string(content.c_str())) {{}}
                    CStringPtr() {{}}
                }};

                CStringPtr create_c_string_ptr(std::string content) {{
                    return {{ content }};
                }}

                CStringPtr create_null_c_string_ptr() {{
                    return {{}};
                }}

                std::string read_c_string_ptr(const CStringPtr& str_ref) {{
                    return {{ str_ref.ptr }};
                }}

                bool is_c_string_ptr_null(const CStringPtr& str_ref) {{
                    return str_ref.ptr == nullptr;
                }}
        "}
        .as_bytes(),
    )
    .unwrap();

    file
}

pub(crate) fn gen(native: &Native) -> String {
    let Native {
        params,
        results,
        name,
        hash,
    } = native;

    let mut cpp_result_params = vec![];
    let cpp_params = params
        .iter()
        .filter_map(|param| {
            let cpp_param = cpp_impls::cpp_param_from_param(param);
            if param.r#ref {
                cpp_result_params.push(cpp_param);
                None
            } else {
                Some(cpp_param)
            }
        })
        .collect::<Vec<_>>();

    let reads: Vec<String> = cpp_params
        .iter()
        .chain(cpp_result_params.iter())
        .filter_map(|v| if v.is_ref() { v.read() } else { None })
        .collect();
    let reads = reads.join("\n    ");

    let mut result_param_declarations: Vec<String> = cpp_result_params
        .iter()
        .map(|v| v.ref_declaration())
        .collect();

    let native_return = cpp_impls::cpp_param_from_param(&Param {
        r#type: results[0].clone(),
        name: NATIVE_RETURN_IDENT.to_string(),
        r#ref: true,
    });
    let native_return_read = native_return.context_return();

    let mut result_declarations = vec![native_return.ref_declaration()];

    result_declarations.append(&mut result_param_declarations);
    let result_declarations = result_declarations
        .into_iter()
        .filter(|v| !v.contains("VOID"))
        .collect::<Vec<String>>()
        .join(", ");

    let param_declarations: Vec<String> = cpp_params
        .iter()
        .map(|v| {
            if v.is_ref() {
                v.ref_declaration()
            } else {
                v.declaration()
            }
        })
        .collect();

    let param_declarations = if !param_declarations.is_empty() {
        let comma_after_result_decls = if !result_declarations.is_empty() {
            ", "
        } else {
            ""
        };

        format!(
            "{comma_after_result_decls}{}",
            param_declarations.join(", ")
        )
    } else {
        String::new()
    };

    let pushes: Vec<String> = cpp_params
        .iter()
        .chain(cpp_result_params.iter())
        .map(|v| {
            v.declaration();
            if v.is_ref() {
                v.context_push_ref().unwrap_or_else(|| v.context_push())
            } else {
                v.context_push()
            }
        })
        .collect();
    let pushes = pushes.join("\n    ");

    formatdoc! {"
        Success {name}({result_declarations}{param_declarations}) {{
            static auto native = alt::ICore::Instance().GetNativeByHash({hash});
            ctx->Reset();

            {pushes}

            Success result = native->Invoke(ctx);
            if (result) {{
                {reads}
                {native_return_read}
            }}
            return result;
        }}
    "}
}

trait CppParam {
    fn is_ref(&self) -> bool;
    fn declaration(&self) -> String;
    fn ref_declaration(&self) -> String;
    fn context_push(&self) -> String;
    fn context_return(&self) -> String;
    fn read(&self) -> Option<String> {
        None
    }
    fn context_push_ref(&self) -> Option<String> {
        None
    }
}

macro_rules! cpp_impls {
    ( $(
        $variant:ident:
        @decl: $declaration:literal,
        @push: $context_push:literal,
        @return: $context_return:literal,
        @ref_decl: $ref_declaration:literal
        $(, @read: $read:literal )?
        $(, @ref_push: $context_push_ref:literal )?
        ;
    )+ ) => {
        $(
            struct $variant {
                name: StdString,
                r#ref: bool,
            }

            impl CppParam for $variant {
                fn is_ref(&self) -> bool {
                    self.r#ref
                }

                fn declaration(&self) -> StdString {
                    formatdoc! {$declaration, self.name}
                }

                fn ref_declaration(&self) -> StdString {
                    formatdoc! {$ref_declaration, self.name}
                }

                fn context_push(&self) -> StdString {
                    formatdoc! {$context_push, self.name}
                }

                fn context_return(&self) -> StdString {
                    formatdoc! {$context_return, self.name}
                }

            $(
                fn read(&self) -> Option<StdString> {
                    Some(formatdoc! {$read, self.name})
                }
            )?

            $(
                fn context_push_ref(&self) -> Option<StdString> {
                    Some(formatdoc! {$context_push_ref, self.name})
                }
            )?
            }
        )+

        pub(super) fn cpp_param_from_param(param: &Param) -> Box<dyn CppParam> {
            match param.r#type {
                $(
                    NativeType::$variant => Box::new($variant {
                        name: param.name.clone(),
                        r#ref: param.r#ref,
                    }),
                )+
            }
        }
    };
}

mod cpp_impls {
    use super::*;

    use std::string::String as StdString;

    cpp_impls!(

        // i32
        Any: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        I32: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        MemoryBuffer: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        Interior: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        Cam: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        FireId: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        Blip: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        Pickup: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        CarGenerator: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        Group: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        Weapon: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        Texture: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        TextureDict: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        CoverPoint: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        TaskSequence: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        ColourIndex: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";
        Sphere: @decl: "i32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultInt();", @ref_decl: "i32* {}";

        // u32
        Hash: @decl: "u32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultUint();", @ref_decl: "u32* {}";
        ScrHandle: @decl: "u32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultUint();", @ref_decl: "u32* {}";
        Object: @decl: "u32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultUint();", @ref_decl: "u32* {}";
        Entity: @decl: "u32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultUint();", @ref_decl: "u32* {}";
        Ped: @decl: "u32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultUint();", @ref_decl: "u32* {}";
        Vehicle: @decl: "u32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultUint();", @ref_decl: "u32* {}";
        Player: @decl: "u32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultUint();", @ref_decl: "u32* {}";
        Train: @decl: "u32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultUint();", @ref_decl: "u32* {}";

        F32: @decl: "f32 {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultFloat();", @ref_decl: "f32* {}";
        Boolean: @decl: "bool {}", @push: "ctx->Push({});", @return: "*{} = ctx->ResultBool();", @ref_decl: "bool* {}";
        Void: @decl: "/* VOID {} */", @push: "/* VOID {} */", @return: "/* VOID {} */", @ref_decl: "/* VOID {} */";
        Vector3:
            @decl: "Vector3Wrapper {}",
            @push: "
                alt::INative::Vector3 alt_{0} {{ {0}.x, 0, {0}.y, 0, {0}.z }};
                ctx->Push(&alt_{0});
            ",
            @return: "
                alt::INative::Vector3 alt_native_return = ctx->ResultVector3();
                {0}.x = alt_native_return.x;
                {0}.y = alt_native_return.y;
                {0}.z = alt_native_return.z;
            ",
            @ref_decl: "Vector3Wrapper& {}",
            @read: "
                {0}.x = alt_{0}.x;
                {0}.y = alt_{0}.y;
                {0}.z = alt_{0}.z;
            ";
        String:
            @decl: "const CStringPtr& {}",
            @push: "ctx->Push({}.ptr);",
            @return: "{}.ptr = clone_c_string(ctx->ResultString());",
            @ref_decl: "CStringPtr& {}",
            @read: "{0}.ptr = alt_{0};",

            // TODO: test this
            @ref_push: "
                char* alt_{0} = save_c_string(\"\"); // creating new empty string because {0} is nullptr
                ctx->Push(alt_{0});
            ";
    );
}
