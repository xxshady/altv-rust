@REM !! requires cargo-watch to be installed
cargo watch --exec build --why --watch altv_module --watch example_resource --watch altvx --watch altv_sdk/src/lib.rs --watch altv_sdk/src/alt_bridge.h --watch altv_sdk/src/callbacks.h --watch altv_sdk/src/runtime.h --watch altv_sdk/build.rs --watch resource_impl --watch logger
