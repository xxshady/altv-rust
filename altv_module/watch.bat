@REM TODO: use proper crate for creating "npm like scripts" 
@REM but for now it doesn't matter

@REM !! requires cargo-watch to be installed

cargo watch --exec build --why --watch ../alt --watch ../resource_api --watch ../altv_sdk/src/lib.rs --watch ../altv_sdk/src/alt_bridge.cpp --watch ../altv_sdk/include/alt_bridge.h --watch .
