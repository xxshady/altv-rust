//! # Reloading
//!
//! Rust module provides reloading feature that can be used for
//! easier development, avoiding server restart and client reconnect
//!
//! > note: currently, it works a bit better on Linux than on Windows.
//!   Background threads spawned by Rust resources are not checked on Windows
//!   which can lead to Undefined Behavior. See also [before_unload](#before_unload).
//!
//! ## How to use
//!
//! > note: ready-made [example](https://github.com/xxshady/altv-rust/tree/release/examples/reloading#how-to-run).
//!
//! To use it:
//!
//! ...compile rust-module with reloading enabled:
//! ```ignore
//! cargo altvup release --reloading
//! ```
//!
//! ...compile your resource with `reloading` feature enabled on `altv` crate,
//! you can do so by adding the following to your Cargo.toml:
//! ```toml
//! [features]
//! reloading = ["altv/reloading"]
//! ```
//!
//! ...and running something like:
//! ```ignore
//! cargo build --features reloading
//! ```
//!
//! ...now start altv-server, run in it:
//! ```ignore
//! stop <your resource name>
//! ```
//!
//! ...recompile your resource and run in altv-server:
//! ```ignore
//! start <your resource name>
//! ```
//!
//! > note: `stop` and `start` are alt:V built-in [commands](https://docs.altv.mp/articles/commandlineargs.html#server-commands).
//!   It's also possible to reload resources programmatically using `altv::Resource::current().stop/start`
//!
//! ## Cleanup at unloading
//!
//! Rust module will cleanup everything owned (created) by Rust resource when it's unloaded:
//! - Destroy all baseobjects (colshapes, vehicles, etc.)
//! - Delete all metadata, including metadata set on connected players
//!
//! > note: connected players do not disappear in a magical way when resource is unloaded,
//!   so be prepared to handle already connected players when resource starts in a different way,
//!   player connect event won't be called for them.
//!   
//! ## `before_unload`
//!
//! Rust module provides [`altv::before_unload`](https://docs.rs/altv/latest/altv/attr.before_unload.html)
//! callback API when something needs to be cleaned up manually when resource is unloaded.
//!
//! ## More details
//!
//! For more details about reloading feature see [relib](https://github.com/xxshady/relib).
//! About caveats, only [*File descriptors and network sockets*](https://github.com/xxshady/relib?tab=readme-ov-file#file-descriptors-and-network-sockets)
//! and [*Dead locks*](https://github.com/xxshady/relib?tab=readme-ov-file#dead-locks) apply to Rust module, ignore the rest.
