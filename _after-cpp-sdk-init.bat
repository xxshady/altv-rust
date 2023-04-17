cd altv_sdk/cpp-sdk/version
call get-version.bat
copy version.h ..\..\cpp-sdk-version.h /Y
cd ../../../cpp_codegen
cargo run
