# `altvup`

**`altvup`** is an installer for the Rust module on [alt:V](https://altv.mp).
It can also download alt:V server binaries & data files.

> If you are familiar with [`altv-pkg`](https://github.com/altmp/altv-pkg), `altvup` is Rust version of it

## How to install

Recommended: (if you don't have it: [cargo-binstall](https://github.com/cargo-bins/cargo-binstall?tab=readme-ov-file#cargo-binaryinstall))<br>
`cargo binstall altvup` 

Or you can compile from source code: (but it's gonna take longer)<br>
`cargo install cargo-altvup`

## How to use

`cargo altvup <branch>` - it will install `rust-module` (more about it [here](#how-rust-module-is-installed)), alt:V server files (`altv-server(.exe)` binary, data bins, etc.) and minimal [server config](https://docs.altv.mp/articles/configs/server.html) if it doesn't exists.

`<branch>` - [alt:V branch](https://docs.altv.mp/articles/branches.html), possible values: release or rc

For example: `cargo altvup release`

There are also some optional parameters:

### `--force-recompile`

Compile latest `rust-module` again (if `rust-module` binary already exists it wont be installed by default).

For example: `cargo altvup release --force-recompile`

### `--src-dir`

The directory to which `altv-rust` source code will be downloaded (`.altvup-src` by default).

For example: `cargo altvup release --src-dir=my_dir`

### `--jsv2`

Download serverside part of [jsv2](https://github.com/altmp/altv-js-module-v2) module (disabled by default).

For example: `cargo altvup release --jsv2`

## How `rust-module` is installed

It downloads `altv-rust` source code to `.altvup-src` directory (see also: [`--src-dir`](#--src-dir)) and compiles `rust-module` (.so or .dll) to the `modules` directory, after compilation `.altvup-src` directory will be deleted.
