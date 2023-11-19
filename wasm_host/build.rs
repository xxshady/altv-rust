fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=../wasm.interface");
    println!("cargo:rerun-if-changed=../wasm_natives.interface");

    build_helper::generate_bindings!(
        host,
        "gen.rs",
        @interfaces
        main: "../wasm.interface"
        extra: [
            "../wasm_natives.interface",
        ]
    );
}
