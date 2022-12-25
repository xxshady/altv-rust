const CPP_SDK_VERSION_DIR: &str = "include/cpp-sdk/version";

fn main() {
    generate_cpp_sdk_version();
    build_rust();

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/alt_bridge.cpp");
    println!("cargo:rerun-if-changed=include/SDK_extend.h");
    println!("cargo:rerun-if-changed=include/cpp-sdk/*.h");
    println!("cargo:rerun-if-changed=include/cpp-sdk/*.cpp");
}

fn generate_cpp_sdk_version() {
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
        .write_to_file("src/cpp-sdk-version.rs")
        .expect("Couldn't write bindings!");
}

fn build_rust() {
    let sources = vec!["src/lib.rs"];
    cxx_build::bridges(sources)
        .file("src/alt_bridge.cpp")
        .flag("-DALT_SERVER_API")
        .flag("/std:c++20")
        .compile("alt_sdk");
}
