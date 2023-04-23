use std::fs;

// TODO: remove these retarded constants
const BASE_OBJECT_TYPE_ENUM_FILE: &str = "cpp-sdk/objects/IBaseObject.h";
const EVENT_TYPE_ENUM_FILE: &str = "cpp-sdk/events/CEvent.h";
const MVALUE_TYPE_ENUM_FILE: &str = "cpp-sdk/types/MValue.h";
const COL_SHAPE_TYPE_ENUM_FILE: &str = "cpp-sdk/script-objects/IColShape.h";
const BLIP_TYPE_ENUM_FILE: &str = "cpp-sdk/script-objects/IBlip.h";
const MARKER_TYPE_ENUM_FILE: &str = "cpp-sdk/script-objects/IMarker.h";
const PLAYER_BODY_PART_ENUM_FILE: &str = "cpp-sdk/events/CWeaponDamageEvent.h";
const PLAYER_CONNECT_DENIED_REASON_ENUM_FILE: &str = "cpp-sdk/events/CPlayerConnectDeniedEvent.h";
const EXPLOSION_TYPE_ENUM_FILE: &str = "cpp-sdk/events/CExplosionEvent.h";
const VEHICLE_MODEL_TYPE_ENUM_FILE: &str = "cpp-sdk/types/VehicleModelInfo.h";

fn main() -> miette::Result<()> {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    generate_cpp_to_rust_bindings(&out_dir);
    build_rust()?;

    Ok(())
}

fn generate_cpp_to_rust_bindings(out_dir: &str) {
    let hash = get_sdk_hash();
    write_sdk_hash(hash, out_dir);

    generate_rust_enum_from_cpp(
        "BaseObjectType",
        "u8",
        BASE_OBJECT_TYPE_ENUM_FILE,
        "enum class Type : uint8_t",
        "base_object_type.rs",
        out_dir,
    );

    generate_rust_enum_from_cpp(
        "EventType",
        "u16",
        EVENT_TYPE_ENUM_FILE,
        "enum class Type : uint16_t",
        "event_type.rs",
        out_dir,
    );

    generate_rust_enum_from_cpp(
        "MValueType",
        "u8",
        MVALUE_TYPE_ENUM_FILE,
        "enum class Type : uint8_t",
        "mvalue_type.rs",
        out_dir,
    );

    generate_rust_enum_from_cpp(
        "ColShapeType",
        "u8",
        COL_SHAPE_TYPE_ENUM_FILE,
        "enum class ColShapeType : uint8_t",
        "col_shape_type.rs",
        out_dir,
    );

    generate_rust_enum_from_cpp(
        "BlipType",
        "u8",
        BLIP_TYPE_ENUM_FILE,
        "enum class BlipType",
        "blip_type.rs",
        out_dir,
    );

    generate_rust_enum_from_cpp(
        "MarkerType",
        "u32",
        MARKER_TYPE_ENUM_FILE,
        "enum class MarkerType",
        "marker_type.rs",
        out_dir,
    );

    generate_rust_enum_from_cpp(
        "PlayerBodyPart",
        "i8",
        PLAYER_BODY_PART_ENUM_FILE,
        "enum class BodyPart : int8_t",
        "player_body_part.rs",
        out_dir,
    );

    generate_rust_enum_from_cpp(
        "PlayerConnectDeniedReason",
        "u8",
        PLAYER_CONNECT_DENIED_REASON_ENUM_FILE,
        "enum Reason: uint8_t",
        "player_connect_denied_reason.rs",
        out_dir,
    );

    generate_rust_enum_from_cpp(
        "ExplosionType",
        "i8",
        EXPLOSION_TYPE_ENUM_FILE,
        "enum class ExplosionType : int8_t",
        "explosion_type.rs",
        out_dir,
    );

    generate_rust_enum_from_cpp(
        "VehicleModelType",
        "u8",
        VEHICLE_MODEL_TYPE_ENUM_FILE,
        "enum class Type : uint8_t",
        "vehicle_model_type.rs",
        out_dir,
    );

    generate_rust_enum_from_cpp(
        "ConfigValueType",
        "u8",
        "cpp-sdk/deps/ConfigBase.h",
        "enum class Type : uint8_t",
        "config_value_type.rs",
        out_dir,
    );
}

fn build_rust() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src");

    let mut build = autocxx_build::Builder::new("src/lib.rs", [&path])
        .extra_clang_args(&["-std=c++20"])
        .build()?;

    let flags = if cfg!(target_os = "windows") {
        ["/std:c++20"]
    } else if cfg!(target_os = "linux") {
        ["-std=c++2a"]
    } else {
        panic!("unsupported target_os");
    };

    for flag in flags {
        build.flag(flag);
    }

    build.compile("altv_sdk");
    Ok(())
}

fn generate_rust_enum_from_cpp(
    enum_name: &str,
    enum_type: &str,
    path: &str,
    str_to_find: &str,
    write_to: &str,
    out_dir: &str,
) {
    let content = String::from_utf8(fs::read(path).unwrap()).unwrap();
    let idx = content.find(str_to_find).unwrap();

    let mut open_brace = false;
    let mut start_idx = 0;
    let mut end_idx = 0;
    let chars = content.get(idx..).unwrap().as_bytes();

    for (idx, char) in chars.iter().enumerate() {
        if open_brace {
            if *char == b'}' {
                end_idx = idx - 1;
                break;
            }

            continue;
        }

        if *char == b'{' {
            open_brace = true;
            start_idx = idx + 1;
        }
    }

    if !open_brace {
        panic!("cannot find open brace of {str_to_find:?}")
    }

    let mut result_string = String::from_utf8_lossy(&chars[start_idx..=end_idx])
        .to_string()
        .split('\n')
        .filter_map(|val| {
            val.get(2..)
                .map(|v| v.trim())
                .and_then(|v| if v.is_empty() { None } else { Some(v) })
        })
        .map(|v| format!("    {}", upper_to_pascal_case(v)))
        .collect::<Vec<String>>()
        .join("\n");

    if result_string.ends_with(',') {
        result_string.remove(result_string.len() - 1);
    }

    fs::write(
        format!("{out_dir}/{write_to}"),
        format!(
            "// auto-generated from build.rs\n\n\
            primitive_enum! {{ {enum_name} {enum_type} ;\n\
                {result_string},\n\
            }}",
        ),
    )
    .unwrap();
}

fn upper_to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();
    result.push(chars.next().unwrap().to_ascii_uppercase());

    let mut next_char_upper = false;
    for c in chars {
        if next_char_upper {
            next_char_upper = false;
            result.push(c.to_ascii_uppercase());
            continue;
        }

        if c == '_' {
            next_char_upper = true;
            continue;
        }

        result.push(c.to_ascii_lowercase());
    }
    result
}

fn get_sdk_hash() -> String {
    const SDH_HASH_STRING_START: &str = "ALT_SDK_VERSION \"";

    let content = fs::read("cpp-sdk-version.h").unwrap();
    let content = String::from_utf8_lossy(&content[..]).to_string();

    let idx = content.find(SDH_HASH_STRING_START).unwrap();
    let start_of_hash = idx + SDH_HASH_STRING_START.len();
    let end_of_hash = start_of_hash + 7;

    content[start_of_hash..end_of_hash].to_string()
}

fn write_sdk_hash(hash: String, out_dir: &str) {
    fs::write(
        format!("{out_dir}/cpp_sdk_version.rs"),
        format!("pub const ALT_SDK_VERSION: &[u8; 8usize] = b\"{hash}\\0\";"),
    )
    .unwrap();
}
