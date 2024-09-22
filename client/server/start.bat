cd ..\rust_server &&^
cargo build &&^
cd .. &&^
npm run build-wasm &&^
npm run generate-sourcemap &&^
npm run build-js &&^
cd server &&^
altv-server.exe
