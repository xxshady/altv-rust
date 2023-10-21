use std::fs;

use statics::{HEADERS_CUSTOM, HEADERS};

use crate::statics::{SUPPORTED_CPP_TYPES, SUPPORTED_CPP_TYPES_IN_CLASSES};

const CPP_CODE_CLIENT_API_START: &str = "#ifdef ALT_CLIENT_API";
const CPP_CODE_ENDIF_DIRECTIVE: &str = "#endif";
const CPP_CODE_ELSE_DIRECTIVE: &str = "#else";
const CPP_CODE_SERVER_API_START: &str = "#ifdef ALT_SERVER_API";
const CPP_OUT_DIR: &str = "../altv_sdk/src/alt_classes";

mod statics;

fn main() {
    fs::remove_dir_all(CPP_OUT_DIR).unwrap();
    fs::create_dir(CPP_OUT_DIR).unwrap();

    for (class, file, custom_caller) in HEADERS_CUSTOM {
        gen(class, file, Some(*custom_caller));
    }

    for (class, file) in HEADERS {
        gen_default(class, file);
    }
}

// below is the most shit coded mess ever...

fn gen_default(class_name: &str, in_file: &str) {
    gen(class_name, in_file, None);
}

fn gen(class_name: &str, in_file: &str, custom_method_caller: Option<fn(String) -> String>) {
    let (class_name, prefix) = if class_name.starts_with("struct ") {
        (class_name.replace("struct ", ""), "struct")
    } else {
        (class_name.to_string(), "class")
    };

    let mut rust_functions_cpp_file = format!(
        "#pragma once\n\
        #define ALT_SERVER_API\n\
        #include \"alt_bridge.h\"\n\n\
        namespace {class_name} {{\n\n"
    );

    let mut cpp_method_to_rust = |method: String| match cpp_method_to_rust_compatible_func(
        &class_name,
        method.clone(),
        custom_method_caller,
    ) {
        Ok(rust_func) => {
            rust_functions_cpp_file += &format!("{rust_func}\n");
        }
        Err(err) => {
            println!("failed to convert class: \"{class_name}\" method: {method}\nto rust func, error: {err:?}");
        }
    };

    let content = String::from_utf8(fs::read(in_file).expect(in_file)).expect(in_file);

    let mut cpp_if_directive = "";
    let mut multiline_method: String = "".to_string();

    let mut is_in_class_block = false;
    let mut is_in_nested_class_block = false;
    let mut nested_class_block_brace = 0;

    for line in get_lines_of_content_and_skip_useless(&content, &class_name, prefix) {
        let mut line = line.trim();
        // dbg!(line);

        if !multiline_method.is_empty() {
            // println!("multiline_method line: {line:#?}");
            multiline_method += line.replace('\n', " ").as_str();

            if line.ends_with(';') {
                // println!("multiline end");
                // println!("full multiline:\n{multiline_method}");

                cpp_method_to_rust(std::mem::replace(&mut multiline_method, "".to_string()));
                multiline_method = "".to_string();
            }
            continue;
        }

        if line.is_empty() {
            // println!("empty line");
            continue;
        }

        if line.starts_with("class ") && !line.ends_with(';') {
            if !is_in_class_block {
                is_in_class_block = true;
            } else {
                // println!("skipping this nested class block");
                is_in_nested_class_block = true;
            }
        } else if is_in_nested_class_block {
            if line.starts_with('{') {
                nested_class_block_brace += 1;
            } else if line.starts_with('}') {
                nested_class_block_brace -= 1;
            }
            if nested_class_block_brace == 0 {
                // println!("end of this nested class block");
                is_in_nested_class_block = false;
            } else {
                // println!("still in nested class block...");
                continue;
            }
        }

        if !cpp_if_directive.is_empty() {
            if line.starts_with(CPP_CODE_ENDIF_DIRECTIVE) {
                // println!("CPP_CODE_CLIENT_API_END");
                cpp_if_directive = "";
            } else {
                match cpp_if_directive {
                    "client_api" => {
                        continue;
                    }
                    "server_api" => {
                        if line.starts_with(CPP_CODE_ELSE_DIRECTIVE) {
                            cpp_if_directive = "client_api";
                            // println!("CPP_CODE_CLIENT_API_START");
                            continue;
                        }
                    }
                    v => panic!("unknown cpp_if_directive: {v}"),
                }
            }
        }

        if line.starts_with(CPP_CODE_CLIENT_API_START) {
            cpp_if_directive = "client_api";
            // println!("CPP_CODE_CLIENT_API_START");
            continue;
        } else if line.starts_with(CPP_CODE_SERVER_API_START) {
            cpp_if_directive = "server_api";
            // println!("CPP_CODE_SERVER_API_START");
        }

        if line.starts_with("virtual ") {
            line = line.get("virtual ".len()..).unwrap();
            // dbg!(line);
        }

        // lets fuck up this shit coded mess even more
        {
            let check_weird_pointer_type = |cpp_type: &str| {
                let mut cpp_type = cpp_type.to_string();
                if !cpp_type.ends_with('*') {
                    return false;
                }

                let cpp_pointer = &format!("{} *", {
                    cpp_type.remove(cpp_type.len() - 1);
                    cpp_type
                });
                line.starts_with(cpp_pointer)
            };

            if !(SUPPORTED_CPP_TYPES.keys().any(|cpp_type| {
                let cpp_type = cpp_type.to_string();
                if check_weird_pointer_type(&cpp_type) {
                    return true;
                }
                line.starts_with(&cpp_type) || line.starts_with(&format!("const {cpp_type}"))
            }) || SUPPORTED_CPP_TYPES_IN_CLASSES.keys().any(|v| {
                let cpp_type = v.replace(&format!("{class_name}::"), "");
                // println!("SUPPORTED_CPP_TYPES_IN_CLASSES check class_name: {class_name} cpp_type: {cpp_type} line: {line}");
                if check_weird_pointer_type(&cpp_type) {
                    return true;
                }
                line.starts_with(&cpp_type) || line.starts_with(&format!("const {v}"))
            })) {
                // println!("either its not method or unsupported cpp type: {line}");
                continue;
            }
        }

        // println!("line: {line:#?}");

        if line.ends_with(',') && line.contains('(') {
            multiline_method = line.to_string();
            // println!("multiline start!");
            continue;
        }
        if !(
            // pure virtual method of object class
            line.ends_with("= 0;") ||
            // normal method of event class
            line.ends_with("; }") || 
            line.ends_with("; };") ||
            line.ends_with(')') ||
            line.ends_with(") const")
        ) {
            // println!("seems like its property? skipping");
            continue;
        }

        cpp_method_to_rust(line.to_string());
    }

    fs::write(
        format!("{CPP_OUT_DIR}/{class_name}.h"),
        rust_functions_cpp_file + "\n} // namespace",
    )
    .unwrap();

    println!("{class_name} done");
}

struct MethodParser {
    content: Vec<u8>,
    current_idx: usize,
}

impl MethodParser {
    pub fn new(content: String) -> Self {
        Self {
            current_idx: 0,
            content: content.into_bytes(),
        }
    }

    pub fn next_char(&mut self) -> Option<&u8> {
        let next = self.content.get(self.current_idx);
        self.current_idx += 1;
        next
    }

    pub fn is_next_char(&self, char: char) -> bool {
        if let Some(c) = self.content.get(self.current_idx) {
            return *c == (char as u8);
        }
        false
    }

    pub fn is_it_last_char(&self) -> bool {
        self.current_idx == self.content.len()
    }
}

#[derive(Debug)]
struct CurrentWord {
    content: String,
}

impl CurrentWord {
    pub fn new() -> Self {
        Self {
            content: Default::default(),
        }
    }

    pub fn reset(&mut self) -> String {
        std::mem::take(&mut self.content)
    }

    pub fn add_char(&mut self, char: u8) {
        self.content += std::str::from_utf8(&[char]).unwrap();
    }
}

#[derive(Debug)]
struct ProcParam {
    pub type_name: Option<String>,
    pub is_const: bool,
    pub name: Option<String>,
}

#[derive(Debug)]
struct CompletedParam {
    pub type_name: String,
    pub is_const: bool,
    pub name: String,
}

#[derive(Debug)]
struct ProcReturnType {
    pub is_const: bool,
    pub type_name: Option<String>,
}

#[derive(Debug)]
struct ReturnType {
    pub is_const: bool,
    pub type_name: String,
}

#[derive(Debug)]
struct CppMethod {
    pub name: String,
    pub return_type: ReturnType,
    pub parameters: Vec<CompletedParam>,
    pub is_const: bool,
}

fn parse_cpp_method(class_name: &str, method: String) -> anyhow::Result<CppMethod> {
    println!("parsing method: {method:?}");

    let mut method_parser = MethodParser::new(method);

    let mut in_word = true;
    let mut current_word = CurrentWord::new();
    let mut return_type = ProcReturnType {
        is_const: false,
        type_name: None,
    };
    let mut method_name: Option<String> = None;
    let mut is_const_method = false;
    let mut parameters_parsing = false;
    let mut proc_parameters: Vec<ProcParam> = vec![];
    let mut next_param_word_ignored = false;
    let mut pointer_param = false;
    let mut pointer_return_type = false;
    let mut generic_type = 0_u8;

    while let Some(char) = method_parser.next_char().copied() {
        // dbg!(char as char);

        if in_word {
            if parameters_parsing && char == b' ' && method_parser.is_next_char('*') {
                // println!("parameters_parsing pointer type: {current_word:?}");
                pointer_param = true;
                continue;
            } else if return_type.type_name.is_none() // return type parsing
                && char == b' ' && method_parser.is_next_char('*')
            {
                // println!("return type parsing pointer type: {current_word:?}");
                pointer_return_type = true;
                continue;
            }

            // dbg!(is_it_delimiter_char(char));
            match (generic_type, char) {
                (1.., b'>') => {
                    generic_type -= 1;
                    println!("generic_type decrease current: {generic_type}");
                }
                (0.., b'<') => {
                    generic_type += 1;
                    println!("generic_type increase current: {generic_type}");
                }
                (_, _) => {
                    // println!(
                    //     "generic_type: {generic_type} char: {:?}",
                    //     std::str::from_utf8(&[char]).unwrap()
                    // );
                }
            }

            if generic_type == 0 && is_it_delimiter_char(char)
                || pointer_param
                || pointer_return_type
                || method_parser.is_it_last_char()
            {
                if pointer_param {
                    // println!("word end pointer_param");
                    pointer_param = false;
                    current_word.add_char(b'*');
                } else if pointer_return_type {
                    // println!("word end pointer_return_type");
                    current_word.add_char(b'*');
                    pointer_return_type = false;
                }

                let word = current_word.reset();
                // println!("word: {word:?}");

                if method_parser.is_it_last_char() && word == "cons" && char == b't' {
                    // println!("const method, end of parsing");
                    is_const_method = true;
                }

                if return_type.is_const && return_type.type_name.is_none() {
                    // println!("const return type set type value: {word:?}");
                    return_type.type_name.replace(word);
                } else if !return_type.is_const && return_type.type_name.is_none() {
                    if word == "const" {
                        // println!("const return type");
                        return_type.is_const = true;
                    } else {
                        // println!("non-const return type");
                        return_type.type_name.replace(word);
                    }
                } else if method_name.is_none() {
                    // println!("set method_name");
                    method_name = Some(word);

                    if char == b'(' && method_parser.is_next_char(')') {
                        // println!("no parameters");
                        // println!("parameters end");
                        parameters_parsing = false;
                        in_word = false;
                        continue;
                    }

                    // println!("parameters_parsing start");
                    parameters_parsing = true;
                } else if parameters_parsing {
                    macro_rules! push_new_param {
                        () => {{
                            let (is_const, type_name) = if word == "const" {
                                (true, None)
                            } else {
                                (false, Some(word))
                            };
                            proc_parameters.push(ProcParam {
                                type_name,
                                name: None,
                                is_const,
                            });
                        };};
                    }

                    if next_param_word_ignored {
                        // println!("ignoring word: {word:?} by next_param_word_ignored");
                        next_param_word_ignored = false;
                    } else if let Some(p) = proc_parameters.last_mut() {
                        if p.name.is_some() {
                            if word == "=" {
                                // println!("ignoring default value of optional parameter");
                                next_param_word_ignored = true;
                                continue;
                            }
                            push_new_param!();
                        } else if p.is_const {
                            if p.type_name.is_none() {
                                p.type_name.replace(word);
                            } else {
                                p.name.replace(word);
                            }
                        } else {
                            p.name.replace(word);
                        }
                    } else {
                        push_new_param!();
                    }

                    if char == b')' {
                        // println!("parameters end");
                        parameters_parsing = false;
                    }
                } else {
                    if word == "const" {
                        // println!("const method");
                        is_const_method = true;
                    }
                    // println!("method parsing end");
                    break;
                }

                in_word = false;
                continue;
            }
            current_word.add_char(char);
        } else if !in_word && !is_it_delimiter_char(char) {
            // println!("starting word with {:?}", char as char);
            current_word.add_char(char);
            in_word = true;
        }
    }

    let mut result_parameters: Vec<CompletedParam> = vec![];

    for p in proc_parameters {
        result_parameters.push(CompletedParam {
            is_const: p.is_const,
            name: p.name.clone().unwrap_or_else(|| {
                panic!(
                    "param name is none (type: {:?}, is_const: {:?})",
                    p.type_name, p.is_const
                )
            }),
            type_name: cpp_to_rust_type(
                class_name,
                &p.type_name.unwrap_or_else(|| {
                    panic!(
                        "param type is none (name: {:?}, is_const: {:?})",
                        p.name, p.is_const
                    )
                }),
            )?,
        })
    }

    Ok(CppMethod {
        name: method_name.unwrap(),
        return_type: ReturnType {
            is_const: return_type.is_const,
            type_name: cpp_to_rust_type(class_name, &return_type.type_name.unwrap())?,
        },
        parameters: result_parameters,
        is_const: is_const_method,
    })
}

fn cpp_to_rust_type(class_name: &str, cpp_type: &str) -> anyhow::Result<String> {
    if let Some(rust_type) = SUPPORTED_CPP_TYPES.get(cpp_type) {
        Ok(rust_type.to_string())
    } else if let Some(rust_type) =
        SUPPORTED_CPP_TYPES_IN_CLASSES.get(format!("{class_name}::{cpp_type}").as_str())
    {
        Ok(rust_type.to_string())
    } else {
        anyhow::bail!("unsupported cpp type: {cpp_type:?}");
    }
}

type RustFuncName = String;
fn cpp_method_to_rust_compatible_func(
    class_name: &str,
    method: String,
    custom_method_caller: Option<impl Fn(String) -> String>,
) -> anyhow::Result<RustFuncName> {
    // dbg!(&method);
    let parsed_method = parse_cpp_method(class_name, method)?;
    // dbg!(&parsed_method);

    let method_name = parsed_method.name;
    let mut params = vec![];
    for p in parsed_method.parameters.iter() {
        params.push({
            let CompletedParam {
                name,
                type_name,
                is_const,
            } = p;
            match type_name.as_str() {
                "Vector3Wrapper" => format!("f32 {name}_x, f32 {name}_y, f32 {name}_z"),
                "Vector2Wrapper" => format!("f32 {name}_x, f32 {name}_y"),
                "alt::Quaternion" => {
                    format!("f32 {name}_x, f32 {name}_y, f32 {name}_z, f32 {name}_w")
                }
                "RGBAWrapper" => format!("u8 {name}_r, u8 {name}_g, u8 {name}_b, u8 {name}_a"),
                "std::vector<WeaponWrapper>" => {
                    "---std::vector<WeaponWrapper> is not implemented as param".to_string()
                }
                "BaseObjectType" => "---BaseObjectType is not implemented as param".to_string(),
                "ColShapeType" => "---ColShapeType is not implemented as param".to_string(),
                "WeaponDamageEventBodyPart" => {
                    "---WeaponDamageEventBodyPart is not implemented as param".to_string()
                }
                "EventType" => format!("u16 {name}"),
                "Vector2Vec" => format!("Vector2Vec {name}"),
                "PlayerConnectDeniedReason" => {
                    "---PlayerConnectDeniedReason is not implemented as param".to_string()
                }
                "ExplosionType" => "---ExplosionType is not implemented as param".to_string(),
                "VoiceConnectionState" => "---VoiceConnectionState is not implemented as param".to_string(),
                "MValueUnorderedMapWrapper" => format!("MValueUnorderedMapWrapper {name}"),
                "alt::AmmoFlags" => {
                    format!("bool {name}_infiniteAmmo, bool {name}_addSmokeOnExplosion, bool {name}_fuse, bool {name}_fixedAfterExplosion")
                }
                _ => format!(
                    "{}{type_name} {name}",
                    (if *is_const { "const " } else { "" }),
                ),
            }
        })
    }
    let params = params.join(", ");

    let passed_params = parsed_method
        .parameters
        .iter()
        .map(|p| {
            let CompletedParam {
                name, type_name, ..
            } = p;
            match type_name.as_str() {
                "MValueMutWrapper" => format!("{name}.ptr"),
                "ConstMValueWrapper" => format!("{name}.ptr"),
                "Vector3Wrapper" => format!("{{ {name}_x, {name}_y, {name}_z }}"),
                "Vector2Wrapper" => format!("{{ {name}_x, {name}_y }}"),
                "alt::Quaternion" => format!("{{ {name}_x, {name}_y, {name}_z, {name}_w }}"),
                "RGBAWrapper" => format!("{{ {name}_r, {name}_g, {name}_b, {name}_a }}"),
                "std::vector<WeaponWrapper>" => {
                    "---std::vector<WeaponWrapper> is not implemented as passed param".to_string()
                }
                "BaseObjectType" => {
                    "---BaseObjectType is not implemented as passed param".to_string()
                }
                "ColShapeType" => "---ColShapeType is not implemented as passed param".to_string(),
                "BlipType" => format!("static_cast<alt::IBlip::BlipType>({name})"),
                "MarkerType" => format!("static_cast<alt::IMarker::MarkerType>({name})"),
                "AmmoSpecialType_t" => format!("static_cast<alt::AmmoSpecialType>({name})"),
                "WeaponDamageEventBodyPart" => {
                    "---WeaponDamageEventBodyPart is not implemented as passed param".to_string()
                }
                "EventType" => format!("static_cast<alt::CEvent::Type>({name})"),
                "Vector2Vec" => format!("{name}.into_alt_vec()"),
                "PlayerConnectDeniedReason" => {
                    "---PlayerConnectDeniedReason is not implemented as passed param".to_string()
                }
                "ExplosionType" => {
                    "---ExplosionType is not implemented as passed param".to_string()
                }
                "VoiceConnectionState" => {
                    "---VoiceConnectionState is not implemented as passed param".to_string()
                }
                "MValueUnorderedMapWrapper" => format!("{name}.value"),
                "MValueUnorderedMapWrapper&" => format!("{name}.value"),
                "PlayerVector" => format!("player_wrapper_vec_to_alt({name})"),
                "alt::AmmoFlags" => format!(
                    "create_ammo_flags_from_params(\n        \
                        {name}_infiniteAmmo,\n        \
                        {name}_addSmokeOnExplosion,\n        \
                        {name}_fuse,\n        \
                        {name}_fixedAfterExplosion\n        \
                    )"
                ),
                "CloudAuthResult_t" => format!("static_cast<alt::CloudAuthResult>({name})"),
                "Benefit_t" => format!("static_cast<alt::Benefit>({name})"),
                _ => name.to_string(),
            }
        })
        .collect::<Vec<String>>()
        .join(", ");
    let params_content = if !params.is_empty() {
        params
    } else {
        "".to_string()
    };

    let return_type = parsed_method.return_type.type_name;
    let extra_wrapper_for_return = match return_type.as_str() {
        "MValueMutWrapper" => |v: &str| {
            format!(
                "MValueMutWrapper wrapper;\n    \
                wrapper.ptr = {v};\n    \
                return wrapper"
            )
        },
        "ConstMValueWrapper" => |v: &str| {
            format!(
                "ConstMValueWrapper wrapper;\n    \
                wrapper.ptr = {v};\n    \
                return wrapper"
            )
        },
        "Vector3Wrapper" => |v: &str| {
            format!(
                "auto vector3 = {v};\n    \
                return {{ vector3[0], vector3[1], vector3[2] }}"
            )
        },
        "Vector2Wrapper" => |v: &str| {
            format!(
                "auto vector2 = {v};\n    \
                return {{ vector2[0], vector2[1] }}"
            )
        },
        "RGBAWrapper" => |v: &str| {
            format!(
                "auto rgba = {v};\n    \
                return {{ rgba.r, rgba.g, rgba.b, rgba.a }}"
            )
        },
        "std::vector<WeaponWrapper>" => |v: &str| {
            format!(
                "auto alt_weapons = {v};\n    \
                std::vector<WeaponWrapper> weapons {{}};\n    \
                weapons.reserve(alt_weapons.size());\n    \
                for (const auto& w : alt_weapons) {{\n        \
                    weapons.push_back({{ w.hash, w.tintIndex, w.components }});\n    \
                }}\n    \
                return weapons"
            )
        },
        "BaseObjectType" => |v: &str| format!("return static_cast<uint8_t>({v})"),
        "ColShapeType" => |v: &str| format!("return static_cast<uint8_t>({v})"),
        "BlipType" => |v: &str| format!("return static_cast<uint8_t>({v})"),
        "MarkerType" => |v: &str| format!("return static_cast<uint32_t>({v})"),
        "AmmoSpecialType_t" => |v: &str| format!("return static_cast<uint32_t>({v})"),
        "WeaponDamageEventBodyPart" => |v: &str| format!("return static_cast<int8_t>({v})"),
        "EventType" => |v: &str| format!("return static_cast<uint16_t>({v})"),
        "StdStringClone" => |v: &str| format!("return std::string {{ {v} }}"),
        "MValueWrapperVec" => |v: &str| {
            format!(
                "auto args = {v};\n    \
                auto mvalue_vec = create_mvalue_vec();\n    \
                for (const auto& e : args) {{\n    \
                    ConstMValueWrapper wrapper;\n    \
                    wrapper.ptr = e;\n    \
                    mvalue_vec.push_back(wrapper.clone());\n    \
                }}\n    \
                return mvalue_vec"
            )
        },
        "PlayerConnectDeniedReason" => |v: &str| format!("return static_cast<uint8_t>({v})"),
        "ExplosionType" => |v: &str| format!("return static_cast<int8_t>({v})"),
        "VoiceConnectionState" => |v: &str| format!("return static_cast<uint8_t>({v})"),
        "std::vector<FireInfoWrapper>" => |v: &str| {
            format!(
                "auto alt_vec = {v};\n    \
                std::vector<FireInfoWrapper> vec {{}};\n    \
                vec.reserve(alt_vec.size());\n    \
                for (const auto& e : alt_vec) {{\n        \
                    vec.push_back({{ {{ e.position[0], e.position[1], e.position[2] }}, e.weaponHash }});\n    \
                }}\n    \
                return vec"
            )
        },
        "alt::VehicleModelInfo*" => |v: &str| format!("return &{v}"),
        "alt::PedModelInfo*" => |v: &str| format!("return &{v}"),
        "alt::WeaponModelInfo*" => |v: &str| format!("return &{v}"),
        "BaseObjectVector" => |v: &str| {
            format!(
                "auto alt_vec = {v};\n    \
                BaseObjectVector vec {{}};\n    \
                vec.reserve(alt_vec.size());\n    \
                for (const auto& e : alt_vec) {{\n        \
                    BaseObjectPtrWrapper wrapper;\n        \
                    wrapper.ptr = std::make_shared<alt::IBaseObject*>(e);\n        \
                    vec.push_back(wrapper.clone());\n    \
                }}\n    \
                return vec"
            )
        },
        "ResourceVector" => |v: &str| {
            format!(
                "auto alt_vec = {v};\n    \
                ResourceVector vec {{}};\n    \
                vec.reserve(alt_vec.size());\n    \
                for (const auto& e : alt_vec) {{\n        \
                    ResourcePtrWrapper wrapper;\n        \
                    wrapper.ptr = std::make_shared<alt::IResource*>(e);\n        \
                    vec.push_back(wrapper.clone());\n    \
                }}\n    \
                return vec"
            )
        },
        "PlayerVector" => |v: &str| {
            format!(
                "auto alt_vec = {v};\n    \
                PlayerVector vec {{}};\n    \
                vec.reserve(alt_vec.size());\n    \
                for (const auto& e : alt_vec) {{\n        \
                    PlayerPtrWrapper wrapper;\n        \
                    wrapper.ptr = std::make_shared<alt::IPlayer*>(e);\n        \
                    vec.push_back(wrapper.clone());\n    \
                }}\n    \
                return vec"
            )
        },
        "std::vector<StreamedEntityWrapper>" => |v: &str| {
            format!(
                "auto alt_vec = {v};\n    \
                std::vector<StreamedEntityWrapper> vec {{}};\n    \
                vec.reserve(alt_vec.size());\n    \
                for (const auto& pair : alt_vec) {{\n        \
                    vec.push_back({{ pair.first, pair.second }});\n    \
                }}\n    \
                return vec"
            )
        },
        "EntityAnimHashPairsWrapper" => |v: &str| {
            format!(
                "EntityAnimHashPairsWrapper wrapper;\n    \
                wrapper.value = {v};\n    \
                return wrapper"
            )
        },
        "Benefit_t" => |v: &str| format!("return static_cast<uint8_t>({v})"),
        "CloudAuthResult_t" => |v: &str| format!("return static_cast<uint8_t>({v})"),
        _ => |v: &str| format!("return {v}"),
    };

    let comma_between_ptr_and_params;
    let const_method_ptr_content;
    let mut return_value;
    let ptr_content;

    let method_calling = format!("{method_name}({passed_params})");

    if let Some(custom_method_caller) = custom_method_caller {
        comma_between_ptr_and_params = "";
        ptr_content = "".to_string();
        return_value = custom_method_caller(method_calling);
    } else {
        return_value = format!("ptr->{method_calling}");
        const_method_ptr_content = if parsed_method.is_const { "const " } else { "" };
        comma_between_ptr_and_params = if !params_content.is_empty() { ", " } else { "" };
        ptr_content = format!("{const_method_ptr_content}alt::{class_name}* ptr");
    }

    return_value = extra_wrapper_for_return(&return_value);

    let return_type_const_content = if parsed_method.return_type.is_const {
        "const "
    } else {
        ""
    };

    Ok(format!(
        "{return_type_const_content}{return_type} {method_name}(\
                {ptr_content}\
                {comma_between_ptr_and_params}\
                {params_content}\
            ) {{\n    {return_value};\n\
            }}"
    ))
}

fn is_it_delimiter_char(char: u8) -> bool {
    char == b' ' || char == b',' || char == b'(' || char == b')'
}

fn get_lines_of_content_and_skip_useless<'a>(
    content: &'a str,
    class_name: &'a str,
    prefix: &'a str,
) -> Vec<&'a str> {
    let class_name_line = format!("{prefix} {class_name}");
    let lines = content
        .lines()
        .skip_while(|line| !line.contains(&class_name_line));

    let mut lines = lines.peekable();
    if lines.peek().is_none() {
        panic!("did not found class_name_line: {class_name_line:?}");
    }

    lines.collect()
}
