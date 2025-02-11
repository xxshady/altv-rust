# How to run

1. Make sure you have installed [altvup](../../altvup/README.md)
2. Run `cargo xtask build` to compile Rust resource, rust-module binary and setup `altv-server` (see [xtask](./xtask/src/main.rs) for source code of this command)
3. Run `altv-server` binary in created altv_server directory
4. Run `restart main` in `altv-server` console to reload Rust resource
5. Try to change something in resource/src
6. Run `cargo xtask rebuild` to recompile Rust resource
7. Run `restart main` in `altv-server` console to reload Rust resource
