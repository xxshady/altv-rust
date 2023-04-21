use std::{collections::HashMap, fs};

const CPP_CODE_CLIENT_API_START: &str = "#ifdef ALT_CLIENT_API";
const CPP_CODE_ENDIF_DIRECTIVE: &str = "#endif";
const CPP_CODE_ELSE_DIRECTIVE: &str = "#else";
const CPP_CODE_SERVER_API_START: &str = "#ifdef ALT_SERVER_API";
const CPP_OUT_DIR: &str = "../altv_sdk/src/alt_classes";

lazy_static::lazy_static! {
    static ref SUPPORTED_CPP_TYPES: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("void", "void"),
            ("bool", "bool"),

            ("uint8_t", "u8"),
            ("uint16_t", "u16"),
            ("uint32_t", "u32"),
            ("uint64_t", "u64"),

            ("int8_t", "i8"),
            ("int16_t", "i16"),
            ("int", "cpp_int"), // why? for some reason sdk uses int and int32_t at the same time
            ("int32_t", "i32"),
            ("int64_t", "i64"),

            ("float", "f32"),
            ("double", "f64"),

            ("std::string", "std::string"),
            ("std::string&", "StdStringClone"),
            ("MValue", "MValueMutWrapper"),
            ("MValueConst", "MValueWrapper"),
            ("MValueArgs&", "MValueWrapperVec"),

            ("IBaseObject*", "alt::IBaseObject*"),
            ("IVehicle*", "alt::IVehicle*"),
            ("IEntity*", "alt::IEntity*"),
            ("IPlayer*", "alt::IPlayer*"),
            ("IColShape*", "alt::IColShape*"),
            ("IResource*", "alt::IResource*"),
            ("ICore*", "alt::ICore*"),
            ("IVirtualEntityGroup*", "alt::IVirtualEntityGroup*"),
            ("IVirtualEntity*", "alt::IVirtualEntity*"),
            ("IConnectionInfo*", "alt::IConnectionInfo*"),
            ("VehicleModelInfo&", "alt::VehicleModelInfo*"),
            ("PedModelInfo&", "alt::PedModelInfo*"),
            ("IWorldObject*", "alt::IWorldObject*"),

            ("alt::Prop", "alt::Prop"),
            ("alt::DlcProp", "alt::DlcProp"),
            ("alt::Cloth", "alt::Cloth"),
            ("alt::DlcCloth", "alt::DlcCloth"),
            ("HeadOverlay", "alt::HeadOverlay"),
            ("HeadBlendData", "alt::HeadBlendData"),
            ("alt::CEvent::Type", "EventType"),

            ("alt::Position", "Vector3Wrapper"),
            ("Position", "Vector3Wrapper"),
            ("Vector3f", "Vector3Wrapper"),
            ("Vector2f", "Vector2Wrapper"),
            ("RGBA", "RGBAWrapper"),
            ("alt::RGBA", "RGBAWrapper"),
            ("std::vector<uint32_t>", "std::vector<u32>"),
            ("std::vector<std::string>", "std::vector<std::string>"),
            ("std::vector<Weapon>", "std::vector<WeaponWrapper>"),
            ("std::vector<std::string>&", "std::vector<std::string>"),
            ("std::vector<Vector2f>", "Vector2Vec"),
            ("std::vector<FireInfo>&", "std::vector<FireInfoWrapper>"),
            ("Quaternion", "alt::Quaternion"),
            ("std::vector<IBaseObject*>", "BaseObjectVector"),
            ("std::vector<IResource*>", "ResourceVector"),

            // TODO: wrappers for these
            // ("std::vector<IVirtualEntity*>", "std::vector<alt::IVirtualEntity*>"),
            // ("std::vector<IVirtualEntityGroup*>", "std::vector<alt::IVirtualEntityGroup*>"),

            ("Rotation", "Vector3Wrapper"),
            ("bool*", "bool*"),

            ("Config::Value::ValuePtr", "Config::Value::ValuePtr"),
        ])
    };

    static ref SUPPORTED_CPP_TYPES_IN_CLASSES: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("IBaseObject::Type", "BaseObjectType"),
            ("IColShape::ColShapeType", "ColShapeType"),
            ("IBlip::BlipType", "BlipType"),
            ("CWeaponDamageEvent::BodyPart", "WeaponDamageEventBodyPart"),
            ("CEvent::Type", "EventType"),
            ("CPlayerConnectDeniedEvent::Reason", "PlayerConnectDeniedReason"),
            ("CExplosionEvent::ExplosionType", "ExplosionType"),
        ])
    };
}

fn main() {
    fs::remove_dir_all(CPP_OUT_DIR).unwrap();
    fs::create_dir(CPP_OUT_DIR).unwrap();

    gen(
        "ICore",
        "../altv_sdk/cpp-sdk/ICore.h",
        Some(|v| format!("alt::ICore::Instance().{v}")),
    );

    // objects
    gen_default("IBaseObject", "../altv_sdk/cpp-sdk/objects/IBaseObject.h");
    gen_default("IWorldObject", "../altv_sdk/cpp-sdk/objects/IWorldObject.h");
    gen_default("IEntity", "../altv_sdk/cpp-sdk/objects/IEntity.h");
    gen_default("IPlayer", "../altv_sdk/cpp-sdk/objects/IPlayer.h");
    gen_default("IVehicle", "../altv_sdk/cpp-sdk/objects/IVehicle.h");
    gen_default(
        "IColShape",
        "../altv_sdk/cpp-sdk/script-objects/IColShape.h",
    );
    gen_default("IBlip", "../altv_sdk/cpp-sdk/script-objects/IBlip.h");
    gen_default(
        "ICheckpoint",
        "../altv_sdk/cpp-sdk/script-objects/ICheckpoint.h",
    );
    gen_default(
        "IVirtualEntityGroup",
        "../altv_sdk/cpp-sdk/script-objects/IVirtualEntityGroup.h",
    );
    gen_default(
        "IVirtualEntity",
        "../altv_sdk/cpp-sdk/script-objects/IVirtualEntity.h",
    );

    gen_default(
        "IConnectionInfo",
        "../altv_sdk/cpp-sdk/types/IConnectionInfo.h",
    );

    gen_default(
        "VehicleModelInfo",
        "../altv_sdk/cpp-sdk/types/VehicleModelInfo.h",
    );

    gen_default("PedModelInfo", "../altv_sdk/cpp-sdk/types/PedModelInfo.h");

    // events
    gen_default("CEvent", "../altv_sdk/cpp-sdk/events/CEvent.h");
    gen_default(
        "CWeaponDamageEvent",
        "../altv_sdk/cpp-sdk/events/CWeaponDamageEvent.h",
    );
    gen_default(
        "CColShapeEvent",
        "../altv_sdk/cpp-sdk/events/CColShapeEvent.h",
    );
    gen_default(
        "CConsoleCommandEvent",
        "../altv_sdk/cpp-sdk/events/CConsoleCommandEvent.h",
    );
    gen_default(
        "CClientScriptEvent",
        "../altv_sdk/cpp-sdk/events/CClientScriptEvent.h",
    );
    gen_default(
        "CServerScriptEvent",
        "../altv_sdk/cpp-sdk/events/CServerScriptEvent.h",
    );
    gen_default(
        "CConnectionQueueAddEvent",
        "../altv_sdk/cpp-sdk/events/CConnectionQueueAddEvent.h",
    );
    gen_default(
        "CConnectionQueueRemoveEvent",
        "../altv_sdk/cpp-sdk/events/CConnectionQueueRemoveEvent.h",
    );

    // entity
    gen_default(
        "CNetOwnerChangeEvent",
        "../altv_sdk/cpp-sdk/events/CNetOwnerChangeEvent.h",
    );

    // player
    gen_default(
        "CPlayerConnectEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerConnectEvent.h",
    );
    gen_default(
        "CPlayerDisconnectEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerDisconnectEvent.h",
    );
    gen_default(
        "CPlayerDeathEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerDeathEvent.h",
    );
    gen_default(
        "CPlayerDamageEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerDamageEvent.h",
    );
    gen_default(
        "CPlayerEnteringVehicleEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerEnteringVehicleEvent.h",
    );
    gen_default(
        "CPlayerEnterVehicleEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerEnterVehicleEvent.h",
    );
    gen_default(
        "CPlayerLeaveVehicleEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerLeaveVehicleEvent.h",
    );
    gen_default(
        "CPlayerChangeAnimationEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerChangeAnimationEvent.h",
    );
    gen_default(
        "CPlayerChangeVehicleSeatEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerChangeVehicleSeatEvent.h",
    );
    gen_default(
        "CPlayerWeaponChangeEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerWeaponChangeEvent.h",
    );
    gen_default(
        "CPlayerConnectDeniedEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerConnectDeniedEvent.h",
    );
    gen_default(
        "CPlayerSpawnEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerSpawnEvent.h",
    );
    gen_default(
        "CPlayerRequestControlEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerRequestControlEvent.h",
    );
    gen_default(
        "CPlayerDimensionChangeEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerDimensionChangeEvent.h",
    );
    gen_default(
        "CPlayerChangeInteriorEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerChangeInteriorEvent.h",
    );

    // vehicle
    gen_default(
        "CVehicleAttachEvent",
        "../altv_sdk/cpp-sdk/events/CVehicleAttachEvent.h",
    );
    gen_default(
        "CVehicleDamageEvent",
        "../altv_sdk/cpp-sdk/events/CVehicleDamageEvent.h",
    );
    gen_default(
        "CVehicleDestroyEvent",
        "../altv_sdk/cpp-sdk/events/CVehicleDestroyEvent.h",
    );
    gen_default(
        "CVehicleDetachEvent",
        "../altv_sdk/cpp-sdk/events/CVehicleDetachEvent.h",
    );
    gen_default(
        "CVehicleHornEvent",
        "../altv_sdk/cpp-sdk/events/CVehicleHornEvent.h",
    );
    gen_default(
        "CVehicleSirenEvent",
        "../altv_sdk/cpp-sdk/events/CVehicleSirenEvent.h",
    );

    gen_default(
        "CStartProjectileEvent",
        "../altv_sdk/cpp-sdk/events/CStartProjectileEvent.h",
    );
    gen_default("CFireEvent", "../altv_sdk/cpp-sdk/events/CFireEvent.h");
    gen_default(
        "CExplosionEvent",
        "../altv_sdk/cpp-sdk/events/CExplosionEvent.h",
    );
    gen_default(
        "CExplosionEvent",
        "../altv_sdk/cpp-sdk/events/CExplosionEvent.h",
    );

    // meta
    gen_default(
        "CMetaChangeEvent",
        "../altv_sdk/cpp-sdk/events/CMetaDataChangeEvent.h",
    );
    gen_default(
        "CGlobalMetaDataChangeEvent",
        "../altv_sdk/cpp-sdk/events/CGlobalMetaDataChangeEvent.h",
    );
    gen_default(
        "CGlobalSyncedMetaDataChangeEvent",
        "../altv_sdk/cpp-sdk/events/CGlobalSyncedMetaDataChangeEvent.h",
    );
    gen_default(
        "CSyncedMetaDataChangeEvent",
        "../altv_sdk/cpp-sdk/events/CSyncedMetaDataChangeEvent.h",
    );
    gen_default(
        "CStreamSyncedMetaDataChangeEvent",
        "../altv_sdk/cpp-sdk/events/CStreamSyncedMetaDataChangeEvent.h",
    );
    gen_default(
        "CLocalMetaDataChangeEvent",
        "../altv_sdk/cpp-sdk/events/CLocalMetaDataChangeEvent.h",
    );

    gen_default(
        "CResourceStopEvent",
        "../altv_sdk/cpp-sdk/events/CResourceStopEvent.h",
    );
    gen_default(
        "CResourceStartEvent",
        "../altv_sdk/cpp-sdk/events/CResourceStartEvent.h",
    );

    gen_default("IResource", "../altv_sdk/cpp-sdk/IResource.h");
}

// below is the most shit coded mess ever...

fn gen_default(class_name: &str, in_file: &str) {
    gen(class_name, in_file, None);
}

fn gen(class_name: &str, in_file: &str, custom_method_caller: Option<fn(String) -> String>) {
    let mut rust_functions_cpp_file = format!(
        "#pragma once\n\
        #define ALT_SERVER_API\n\
        #include \"alt_bridge.h\"\n\n\
        namespace {class_name} {{\n\n"
    );

    let mut cpp_method_to_rust = |method: String| match cpp_method_to_rust_compatible_func(
        class_name,
        method.clone(),
        custom_method_caller,
    ) {
        Ok(rust_func) => {
            rust_functions_cpp_file += &format!("{rust_func}\n");
        }
        Err(err) => {
            println!("failed to convert:\n{class_name}: {method}\nto rust func, error: {err:?}");
        }
    };

    let content = String::from_utf8(fs::read(in_file).unwrap()).unwrap();

    let mut cpp_if_directive = "";
    let mut multiline_method: String = "".to_string();

    let mut is_in_class_block = false;
    let mut is_in_nested_class_block = false;
    let mut nested_class_block_brace = 0;

    for line in content.lines() {
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

            if is_it_delimiter_char(char)
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
                "BlipType" => "---BlipType is not implemented as param".to_string(),
                "WeaponDamageEventBodyPart" => {
                    "---WeaponDamageEventBodyPart is not implemented as param".to_string()
                }
                "EventType" => format!("u16 {name}"),
                "Vector2Vec" => format!("Vector2Vec {name}"),
                "PlayerConnectDeniedReason" => {
                    "---PlayerConnectDeniedReason is not implemented as param".to_string()
                }
                "ExplosionType" => "---ExplosionType is not implemented as param".to_string(),
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
                "MValueMutWrapper" => format!("*({name}.ptr)"),
                "MValueWrapper" => format!("*({name}.ptr)"),
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
                "BlipType" => "---BlipType is not implemented as passed param".to_string(),
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
                wrapper.ptr = std::make_shared<alt::MValue>({v});\n    \
                return wrapper"
            )
        },
        "MValueWrapper" => |v: &str| {
            format!(
                "MValueWrapper wrapper;\n    \
                wrapper.ptr = std::make_shared<alt::MValueConst>({v});\n    \
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
        "WeaponDamageEventBodyPart" => |v: &str| format!("return static_cast<int8_t>({v})"),
        "EventType" => |v: &str| format!("return static_cast<uint16_t>({v})"),
        "StdStringClone" => |v: &str| format!("return std::string {{ {v} }}"),
        "MValueWrapperVec" => |v: &str| {
            format!(
                "auto args = {v};\n    \
                auto mvalue_vec = create_mvalue_vec();\n    \
                for (const auto& e : args) {{\n    \
                    MValueWrapper wrapper;\n    \
                    wrapper.ptr = std::make_shared<alt::MValueConst>(e);\n    \
                    mvalue_vec.push_back(wrapper.clone());\n    \
                }}\n    \
                return mvalue_vec"
            )
        },
        "PlayerConnectDeniedReason" => |v: &str| format!("return static_cast<uint8_t>({v})"),
        "ExplosionType" => |v: &str| format!("return static_cast<int8_t>({v})"),
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
