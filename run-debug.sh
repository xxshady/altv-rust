git submodule update --init --recursive
LOG_LEVEL="debug" cargo build
cd altv-server
npm init -y
npm i altv-pkg
npx altv-pkg dev
cp ../target/debug/libaltv_module.so modules/rust.so
cp ../target/debug/libexample_resource.so resources/rust/main.so
chmod +x ./altv-server
./altv-server
