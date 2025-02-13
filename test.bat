set RUST_BACKTRACE=1
set LOG_LEVEL=debug
cargo build --workspace --features reloading &&^
echo building reloading example &&^
cd examples/reloading &&^
echo running `cargo xtask build` &&^
cargo xtask build &&^
echo running altv server &&^
cd altv_server &&^
altv-server.exe
