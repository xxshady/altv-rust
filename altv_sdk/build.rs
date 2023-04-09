use std::fs;

const CPP_SDK_VERSION_DIR: &str = "cpp-sdk/version";
const BASE_OBJECT_TYPE_ENUM_FILE: &str = "cpp-sdk/objects/IBaseObject.h";
const EVENT_TYPE_ENUM_FILE: &str = "cpp-sdk/events/CEvent.h";
const MVALUE_TYPE_ENUM_FILE: &str = "cpp-sdk/types/MValue.h";
const COL_SHAPE_TYPE_ENUM_FILE: &str = "cpp-sdk/script-objects/IColShape.h";
const PLAYER_BODY_PART_ENUM_FILE: &str = "cpp-sdk/events/CWeaponDamageEvent.h";
const PLAYER_CONNECT_DENIED_REASON_ENUM_FILE: &str = "cpp-sdk/events/CPlayerConnectDeniedEvent.h";
const EXPLOSION_TYPE_ENUM_FILE: &str = "cpp-sdk/events/CExplosionEvent.h";

fn main() -> miette::Result<()> {
    generate_cpp_to_rust_bindings();
    build_rust()?;

    rerun_except::rerun_except(&[
        "src/cpp_sdk_version.rs",
        "src/base_object_type.rs",
        "src/event_type.rs",
        "src/mvalue_type.rs",
        "src/col_shape_type.rs",
        "src/player_body_part.rs",
        "src/player_connect_denied_reason.rs",
        "src/explosion_type.rs",
    ])
    .expect("rerun_except failed");

    Ok(())
}

fn generate_cpp_to_rust_bindings() {
    let version_script_path = if cfg!(target_os = "windows") {
        std::fs::canonicalize(format!("{CPP_SDK_VERSION_DIR}/get-version.bat"))
            .unwrap()
            .display()
            .to_string()
    } else if cfg!(target_os = "linux") {
        "./get-version.sh".to_string()
    } else {
        panic!("unsupported target_os");
    };

    if cfg!(target_os = "linux") {
        std::process::Command::new("chmod")
            .current_dir(CPP_SDK_VERSION_DIR)
            .arg("+x")
            .arg(version_script_path.clone())
            .output()
            .unwrap_or_else(|e| {
                panic!(
                    "failed to run chmod +x for get-version script in: {version_script_path:?} {e}"
                )
            });
    }

    std::process::Command::new(version_script_path.clone())
        .current_dir(CPP_SDK_VERSION_DIR)
        .output()
        .unwrap_or_else(|e| {
            panic!("failed to run cpp-sdk get-version script in: {version_script_path:?} {e}")
        });

    let cpp_sdk_version_bindings = bindgen::Builder::default()
        .header(format!("{CPP_SDK_VERSION_DIR}/version.h"))
        .clang_arg("-std=c++17")
        .clang_arg("-xc++")
        .generate()
        .expect("Unable to generate bindings for version.h");

    cpp_sdk_version_bindings
        .write_to_file("src/cpp_sdk_version.rs")
        .expect("Couldn't write bindings!");

    generate_rust_enum_from_cpp(
        "BaseObjectType",
        "u8",
        BASE_OBJECT_TYPE_ENUM_FILE,
        "enum class Type : uint8_t",
        "src/base_object_type.rs",
    );

    generate_rust_enum_from_cpp(
        "EventType",
        "u16",
        EVENT_TYPE_ENUM_FILE,
        "enum class Type : uint16_t",
        "src/event_type.rs",
    );

    generate_rust_enum_from_cpp(
        "MValueType",
        "u8",
        MVALUE_TYPE_ENUM_FILE,
        "enum class Type : uint8_t",
        "src/mvalue_type.rs",
    );

    generate_rust_enum_from_cpp(
        "ColShapeType",
        "u8",
        COL_SHAPE_TYPE_ENUM_FILE,
        "enum class ColShapeType : uint8_t",
        "src/col_shape_type.rs",
    );

    generate_rust_enum_from_cpp(
        "PlayerBodyPart",
        "i8",
        PLAYER_BODY_PART_ENUM_FILE,
        "enum class BodyPart : int8_t",
        "src/player_body_part.rs",
    );

    generate_rust_enum_from_cpp(
        "PlayerConnectDeniedReason",
        "u8",
        PLAYER_CONNECT_DENIED_REASON_ENUM_FILE,
        "enum Reason: uint8_t",
        "src/player_connect_denied_reason.rs",
    );

    generate_rust_enum_from_cpp(
        "ExplosionType",
        "i8",
        EXPLOSION_TYPE_ENUM_FILE,
        "enum class ExplosionType : int8_t",
        "src/explosion_type.rs",
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
        write_to,
        format!(
            "// auto-generated from build.rs\n\n\
            use primitive_enum::primitive_enum;\n\
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
