<div align="center">
<img width="70px" src="https://user-images.githubusercontent.com/54737754/232321923-66ba765e-33a4-449e-9e9b-2dc13ff8c176.svg"/> <img width="80px" src="https://user-images.githubusercontent.com/54737754/232321872-45100319-28a3-46e9-adf9-3dba5b8da9a8.png"/>
</div>
<br>

# Server-side Rust API for alt:V
[![crates.io](https://img.shields.io/crates/v/altv.svg)](https://crates.io/crates/altv)

New server-side [Rust](https://www.rust-lang.org) module for [alt:V](https://altv.mp) platform (WIP)

**Big** thanks to the [creator](https://github.com/justdimaa) of the [first Rust module](https://github.com/justdimaa/altv-rs), as their work helped me understand how to start my own module

## How to use

Before start writing your server-side in Rust you need to [install LLVM](https://rust-lang.github.io/rust-bindgen/requirements.html#installing-clang)
> if you are on Windows, don't forget to set `LIBCLANG_PATH` as an environment variable

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
fn main() {
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

11. Now you can download rust-module `.dll` or `.so` from [latest release](https://github.com/xxshady/altv-rust/releases)

12. Copy it to `modules` directory of your server

13. Add `rust-module` to [`server.toml`](https://docs.altv.mp/articles/configs/server.html) like that:
```toml
modules = ['rust-module']
```

14. Now if you have done everything correctly, you should see green "hello world" message in server console
