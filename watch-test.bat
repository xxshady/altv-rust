@REM !! requires cargo-watch to be installed

set ALTV_RUST_LOG_LEVEL=info
cargo watch --exec "run --bin run_test" --why --watch altv_module --watch test --watch altv --watch altv_sdk/src/helpers.rs --watch altv_sdk/src/alt_classes --watch altv_sdk/src/lib.rs --watch altv_sdk/src/alt_bridge.h --watch altv_sdk/src/callbacks.h --watch altv_sdk/src/runtime.h --watch altv_sdk/build.rs --watch logger --watch core_shared --watch core_resource --watch core_module --watch resource_main_macro --watch cpp_codegen --watch mvalue
