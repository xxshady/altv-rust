@REM TODO: use proper crate for creating "npm like scripts" 
@REM but for now it doesn't matter

@REM !! requires cargo-watch to be installed

cargo watch --exec build --why --watch ../altvx --watch ../altv_sdk/src/lib.rs --watch ../altv_sdk/src/alt_bridge.h --watch ../altv_sdk/src/callbacks.h --watch ../altv_sdk/src/runtime.h --watch ../resource_impl --watch . --watch ../logger
