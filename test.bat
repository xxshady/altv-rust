set RUST_BACKTRACE=1
cargo build --workspace --features reloading
cd examples/reloading
cargo xtask build
cd altv_server
altv-server.exe
