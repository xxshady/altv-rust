git submodule update --init --recursive altv_sdk/cpp-sdk
chmod +x ./_after-cpp-sdk-init.sh
chmod +x ./altv_sdk/cpp-sdk/version/get-version.sh
./_after-cpp-sdk-init.sh
