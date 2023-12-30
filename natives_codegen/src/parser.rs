use std::{collections::BTreeMap, fs};

use serde_json::Value;
use check_keyword::CheckKeyword;

use crate::helpers::ascii_camel_or_pascal_to_snake_case;

#[derive(serde::Deserialize, Debug, Clone)]
pub(crate) enum NativeType {
    Any,
    Blip,
    #[serde(alias = "boolean")]
    Boolean,
    Cam,
    Entity,
    FireId,
    #[serde(alias = "float")]
    F32,
    Hash,
    #[serde(alias = "int")]
    I32,
    Interior,
    MemoryBuffer,
    Object,
    Ped,
    Pickup,
    Player,
    ScrHandle,
    #[serde(alias = "string")]
    String,
    Vector3,
    Vehicle,
    #[serde(alias = "void")]
    Void,

    // these are unused anyway
    // VoidAny,
    // VoidVector3Hash,
    CarGenerator,
    Group,
    Train,
    Weapon,
    Texture,
    TextureDict,
    CoverPoint,
    // this is unused too
    // Camera,
    TaskSequence,
    ColourIndex,
    Sphere,
}

#[derive(serde::Deserialize, Debug)]
struct RawNative {
    params: Vec<RawParam>,
    #[serde(alias = "altName")]
    pub alt_name: String,

    // Not JSON format, needs to be parsed separately
    results: String,
}

pub(crate) struct Native {
    pub name: String,
    pub hash: String,
    pub params: Vec<Param>,

    // Not JSON format, needs to be parsed separately
    pub results: Vec<NativeType>,
}

#[derive(serde::Deserialize, Debug, Clone)]
pub(crate) struct RawParam {
    pub r#type: NativeType,
    pub name: String,
    pub r#ref: bool,
}

#[derive(Clone)]
pub(crate) struct Param {
    pub r#type: NativeType,
    pub internal_name: String,
    pub rust_name: String,
    pub r#ref: bool,
}

pub(crate) fn parse(local_file_path: &str, remote_file_path: &str) -> Vec<Native> {
    let mut parsed = vec![];

    let namespaces = {
        let local_file = fs::read(local_file_path);

        let data: Value = if let Ok(data) = local_file {
            serde_json::from_str(std::str::from_utf8(&data).unwrap()).unwrap()
        } else {
            println!(
                "Cannot read local file at: {local_file_path:?}, downloading from {remote_file_path}..."
            );
            let data: Value = reqwest::blocking::get(remote_file_path)
                .unwrap()
                .json()
                .unwrap_or_else(|e| {
                    panic!("Failed to parse json error: {e}");
                });

            fs::write(local_file_path, data.to_string()).unwrap();
            data
        };

        let Value::Object(namespaces) = data else {
            panic!("Expected object");
        };

        let namespace_names = namespaces.keys().collect::<Vec<_>>();
        dbg!(namespace_names.len());

        namespaces
    };

    for (_namespace, natives) in namespaces {
        type NativeHash = String;
        let natives: BTreeMap<NativeHash, RawNative> = serde_json::from_value(natives).unwrap();
        let natives = natives.into_iter().collect::<Vec<(NativeHash, _)>>();

        for (hash, raw_native) in natives {
            let mut results: Vec<NativeType> = vec![];

            if raw_native.results.contains('[') {
                let mut current_type = String::new();
                for c in raw_native.results.chars().skip(1) {
                    match c {
                        ',' | ']' => {
                            let value_type = std::mem::take(&mut current_type);
                            let value_type = value_type.trim();
                            // dbg!(&value_type);
                            results
                                .push(serde_json::from_str(&format!("\"{value_type}\"")).unwrap());
                        }
                        _ => {
                            current_type += &c.to_string();
                        }
                    }
                }
            } else {
                results.push(serde_json::from_str(&format!("\"{}\"", raw_native.results)).unwrap());
            }

            let params = raw_native
                .params
                .into_iter()
                .map(|v| Param {
                    // adding '_' to param names to avoid name collision with internal params and keywords
                    internal_name: format!("{}_", ascii_camel_or_pascal_to_snake_case(&v.name)),
                    rust_name: ascii_camel_or_pascal_to_snake_case(&v.name).into_safe(),
                    r#ref: v.r#ref,
                    r#type: v.r#type,
                })
                .collect();

            let native = Native {
                name: ascii_camel_or_pascal_to_snake_case(&raw_native.alt_name),
                hash,
                params,
                results,
            };

            parsed.push(native);
        }
    }

    parsed
}
