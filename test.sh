cargo build --workspace --features reloading &&
echo building reloading example &&
cd examples/reloading &&
echo running `cargo xtask build` &&
cargo xtask build &&
echo running altv server &&
cd altv_server &&
chmod +x ./altv-server &&
chmod +x ./altv-crash-handler &&
RUST_BACKTRACE=1 ./altv-server
