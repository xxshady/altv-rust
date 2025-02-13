# alt:V API for Rust

<div align="center">
  <img height="150px" src="https://github.com/user-attachments/assets/f6ef83ce-9833-4c80-a740-bbed4fcdd4c3"/>
</div>

[![crates.io](https://img.shields.io/crates/v/altv.svg)](https://crates.io/crates/altv)

```rust
altv::events::on_player_connect(|event| {
  let name = event.player.name()?;
  altv::log!("player with name: {name} connected!");
  Ok(())
});
```

New server-side [Rust](https://www.rust-lang.org) module for [alt:V](https://altv.mp) platform

**Big** thanks to the [creator](https://github.com/justdimaa) of the [first Rust module](https://github.com/justdimaa/altv-rs), as their work helped me understand how to start my own module

## Client-side part

At first it was [native implementation](https://github.com/xxshady/altv-rust/tree/clientside-shit) using [wasmtime](https://wasmtime.dev/) without JavaScript.
[It worked](https://youtu.be/6HoV71wxJ8I), but because alt:V does not allow you to use custom client-side modules (.dll)
in production without approval, integration into the client core, constant maintenance and more than 0 people using this module,
I switched to a more realistic approach, [JavaScript WASM](https://github.com/xxshady/altv-esbuild-rust-wasm/tree/clientside-api-prototype)

## Docs

API documentation can be found [here](https://docs.rs/altv)

## How to use

First you need to [install](https://rust-lang.github.io/rust-bindgen/requirements.html#installing-clang) LLVM because it's required by autocxx crate

> [!WARNING]
> Currently on Windows latest version of LLVM [doesn't work](https://github.com/google/autocxx/issues/1327#issuecomment-2075460893) with Rust module, you need to install 17.0.1, for example with winget you can do it using this command `winget install LLVM.LLVM --version 17.0.1` (add `--force` if it fails)

> [!IMPORTANT]
> On Windows set LIBCLANG_PATH as an environment variable pointing to the bin directory of your LLVM install. For example, if you installed LLVM to D:\programs\LLVM, then you'd set the value to be D:\programs\LLVM\bin. You also need to have installed Visual Studio with MSVC compiler (usually installed with Rust using Rustup)

> [!NOTE]
> If you have similar error: `src/alt_bridge.h:5:10: fatal error: 'memory' file not found` when installing or building altv_internal_sdk, try [this](https://stackoverflow.com/questions/26333823/clang-doesnt-see-basic-headers/75546125#75546125)

<!-- // TODO: update it -->
<!-- [Video format of this tutorial](https://youtu.be/PRIJsRdjiGg) if you are more into video tutorials -->

1. Use [`altvup`](./altvup/README.md) to install `rust-module` binary and alt:V server files

2. Create new cargo package with `cargo new altv-resource --lib`

3. Configure cargo to compile your crate as dynamic library in your `Cargo.toml`

```toml
[lib]
crate-type = ['cdylib']
```

4. After that you can install [`altv`](https://crates.io/crates/altv) crate with: `cargo add altv`

5. Next step will be to add main function to your resource (`src/lib.rs`)

```rust
use altv::prelude::*;

#[altv::main] // This is required
fn main() -> impl altv::IntoVoidResult {
  altv::log!("~gl~hello world");
}
```

6. Now you can build your resource with `cargo build`

7. In `target/debug/` you should see the dynamic library (`.dll` or `.so`) you just compiled (if you don't see it, make sure you set `lib.crate-type` to `['cdylib']`, see step 3)

8. Create new alt:V resource, in `resources` directory of your server

9. Copy compiled dynamic library (`.dll` or `.so`) to resource directory

10. Create [`resource.toml`](https://docs.altv.mp/articles/configs/resource.html) with this content:

```toml
type = 'rs'

# your compiled crate as .dll or .so
main = 'example.so'
# note: if you are developing on windows and your production server is running
#       on linux you can use .module extension for your file (so here it will be example.module) 
#       and then there will be no need to change resource.toml between linux and windows
```

11. Don't forget to add resource to [`server.toml`](https://docs.altv.mp/articles/configs/server.html)

```toml
# ...
resources = ['your-rust-resource']
# ...
```

12. Now you can start `altv-server`

> [!NOTE]
> If you are on Linux don't forget to run `chmod +x` for `altv-server` and `altv-crash-handler`:

```shell
chmod +x altv-server
chmod +x altv-crash-handler
```

13. If you have done everything correctly, you should see green "hello world" message in the console

14. For development purposes it's also possible to [reload](https://docs.rs/altv/latest/altv/reloading_docs/index.html) Rust resources dynamically.
