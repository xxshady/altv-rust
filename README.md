<div align="center">
<img width="70px" src="https://user-images.githubusercontent.com/54737754/232321923-66ba765e-33a4-449e-9e9b-2dc13ff8c176.svg"/> <img width="80px" src="https://user-images.githubusercontent.com/54737754/232321872-45100319-28a3-46e9-adf9-3dba5b8da9a8.png"/>
</div>
<br>

# Server-side alt:V API for Rust

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

Before all this, you need to [install LLVM](https://rust-lang.github.io/rust-bindgen/requirements.html#installing-clang)<br>

> [!WARNING]
> Currently on Windows latest version of LLVM [doesn't work](https://github.com/google/autocxx/issues/1327#issuecomment-2075460893) with Rust module, you need to install 16.0.6, for example with winget you can do it using this command `winget install LLVM.LLVM --version 16.0.6`

> [!IMPORTANT]
> On Windows set LIBCLANG_PATH as an environment variable pointing to the bin directory of your LLVM install. For example, if you installed LLVM to D:\programs\LLVM, then you'd set the value to be D:\programs\LLVM\bin. You also need to have installed Visual Studio with MSVC compiler (usually installed with Rust using Rustup)

> [!NOTE]
> If you have similar error: `src/alt_bridge.h:5:10: fatal error: 'memory' file not found` when installing or building altv_internal_sdk, try [this](https://stackoverflow.com/questions/26333823/clang-doesnt-see-basic-headers/75546125#75546125)

[Video format of this tutorial](https://youtu.be/PRIJsRdjiGg) if you are more into video tutorials

1. Create new cargo package with `cargo new altv-resource --lib`

2. Configure cargo to compile your crate as `cdylib` in your `Cargo.toml`

```toml
[lib]
crate-type = ['cdylib']
```

3. After that you can install [`altv`](https://crates.io/crates/altv) crate with: `cargo add altv`

4. Next step will be to add main function to your resource (`src/lib.rs`)

```rust
use altv::prelude::*; // Entity, WorldObject traits

#[altv::main] // This is required
fn main() -> impl altv::IntoVoidResult {
    altv::log!("~gl~hello world");
}
```

5. Now you can build your resource with `cargo build`

6. In `target/debug/` you should see the `.dll` or `.so` you just compiled (if you don't see it, make sure you set `lib.crate-type` to `["cdylib"]`, see step 2)

7. Create new alt:V resource, in `resources` directory of your server

8. Copy compiled `.dll` or `.so` to resource directory

9. Create [`resource.toml`](https://docs.altv.mp/articles/configs/resource.html) with this content:

```toml
type = 'rs'
main = 'example.dll' # your compiled .dll or .so
```

10. Don't forget to add resource to [`server.toml`](https://docs.altv.mp/articles/configs/server.html)

11. Now you can download rust-module `.dll` or `.so` from [latest release](https://github.com/xxshady/altv-rust/releases) or with [`cargo-altvup`](https://github.com/xxshady/cargo-altvup)

12. Copy it to `modules` directory of your server (if you do not use [`cargo-altvup`](https://github.com/xxshady/cargo-altvup))

13. Add `rust-module` to [`server.toml`](https://docs.altv.mp/articles/configs/server.html) like that:

```toml
modules = ['rust-module']
```

14. Now if you have done everything correctly, you should see green "hello world" message in server console
