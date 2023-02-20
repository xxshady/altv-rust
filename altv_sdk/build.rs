use std::fs;

const CPP_SDK_VERSION_DIR: &str = "include/cpp-sdk/version";
const BASE_OBJECT_TYPE_ENUM_FILE: &str = "include/cpp-sdk/objects/IBaseObject.h";
const EVENT_TYPE_ENUM_FILE: &str = "include/cpp-sdk/events/CEvent.h";

fn main() {
    generate_cpp_to_rust_bindings();
    build_rust();

    rerun_except::rerun_except(&["src/cpp_sdk_version.rs"]).expect("rerun_except failed");
}

fn generate_cpp_to_rust_bindings() {
    std::process::Command::new(format!("{CPP_SDK_VERSION_DIR}/get-version.bat"))
        .current_dir(CPP_SDK_VERSION_DIR)
        .output()
        .expect("failed to run cpp-sdk get-version.bat");

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
}

fn build_rust() {
    let sources = vec!["src/lib.rs"];
    cxx_build::bridges(sources)
        .file("src/alt_bridge.cpp")
        .flag("-DALT_SERVER_API")
        .flag("/std:c++20")
        .compile("alt_sdk");
}

fn generate_rust_enum_from_cpp(
    enum_name: &str,
    enum_type: &str,
    path: &str,
    str_to_find: &str,
    write_to: &str,
) {
    let content = String::from_utf8(fs::read(path).unwrap()).unwrap();
    let str = str_to_find;
    let idx = content.find(str).unwrap();

    let mut open_brace = false;
    let mut start_idx = 0;
    let mut end_idx = 0;
    let chars = content.get(idx..).unwrap().as_bytes();

    for (idx, char) in chars.iter().enumerate() {
        if open_brace {
            // println!("");
            if *char == b'}' {
                end_idx = idx - 1;
                break;
            }

            continue;
        }

        if *char == b'{' {
            open_brace = true;
            start_idx = idx + 1;
            println!("open brace found at {idx:?}");
        }
    }

    if !open_brace {
        panic!("cannot find open brace of {str:?}")
    }

    let result_string = String::from_utf8_lossy(&chars[start_idx..=end_idx])
        .to_string()
        .split('\n')
        .filter_map(|val| val.get(2..))
        .collect::<Vec<&str>>()
        .join("\n");

    fs::write(
        write_to,
        format!(
            "#![allow(non_camel_case_types)]\n\
            use primitive_enum::primitive_enum;\n\
            primitive_enum! {{ {enum_name} {enum_type} ;\n\
                {result_string},\n\
            }}",
        ),
    )
    .unwrap();
}
