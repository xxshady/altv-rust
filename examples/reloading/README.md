# How to run

> note: see [reloading](https://docs.rs/altv/latest/altv/reloading_docs/index.html) docs

1. Make sure you have installed [altvup](../../altvup/README.md)
2. Run `cargo xtask build` to compile Rust resource, rust-module binary and setup `altv-server` (see [xtask](./xtask/src/main.rs) for source code of this command)
3. Run `altv-server` binary in created altv_server directory
4. Run `restart main` in `altv-server` console to reload resource
5. Try to change something in resource/src
6. Run `stop main` in `altv-server` console to stop resource
6. Run `cargo xtask rebuild` to recompile resource
7. Run `start main` in `altv-server` console to start resource with updated code
