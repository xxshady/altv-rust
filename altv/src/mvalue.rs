//! MValue, stands for Multi Value, this is needed so you can tell alt:V
//! how to understand Rust types in more general way
//! to send or receive something from clients (players), or communicate with
//! [other alt:V resources](https://docs.altv.mp/articles/resources.html#types).
//!
//! For example, JavaScript has type [Object](https://javascript.info/object),
//! Rust does not, but has a [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html),
//! so alt:V has Dict MValue type, which is `HashMap<String, MValue>` in Rust API, and `Object` in JS API
//!
//! If you want to know more about MValue, you can read more [here](https://docs.altv.mp/sdk/mvalues.html).
//!
//! # Option type
//! Since there is no MValue::Option, Rust [`Option`](https://doc.rust-lang.org/std/option) type
//! is handled differently when it is [converted to MValue](https://github.com/xxshady/altv-rust/blob/380df752ff46c87887b785635a55ac190224577c/core_resource/src/mvalue.rs#L113-L119).
//!
//! ```rust
//! let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();
//!
//! let bool: Option<bool> = Some(true);
//! // Here it is converted to MValue::Bool
//! vehicle.set_meta("example", bool).unwrap();
//!
//! // outputs "Bool(true)"
//! dbg!(vehicle.get_meta("example").unwrap());
//!
//! let none: Option<bool> = None;
//! // Here it is converted to MValue::None
//! vehicle.set_meta("example", none).unwrap();
//!
//! // outputs "None"
//! dbg!(vehicle.get_meta("example").unwrap());
//! ```
//!
//! # Examples
//!
//! `altv::mvalue::list` creates `MValue::List` (dynamic array or vector of mvalues).
//!
//! ```rust
//! // Sending local "example" event with one list as argument
//! altv::events::emit!("example", altv::mvalue::list![1, 2, 3].unwrap()).unwrap();
//!
//! // Applying a per-resource vehicle meta
//! let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();
//! vehicle.set_meta("example", altv::mvalue::list![1, 2, 3].unwrap()).unwrap();
//! ```
//!
//! How it can be handled in JavaScript:
//! ```js
//! alt.on("example", (list) => {
//!     alt.log("is array:", Array.isArray(list)) // is array: true
//!     alt.log("list:", list) // list: [ 1, 2, 3 ]
//! })
//!
//! // Same for vehicle meta:
//! const vehicle = alt.Vehicle.all[0]
//! const list = vehicle.getMeta("example")
//! alt.log("list:", list) // list: [ 1, 2, 3 ]
//! ```
//!
//! `altv::mvalue::dict` creates `MValue::Dict` (`HashMap<String, MValue>` in Rust).
//!
//! ```rust
//! // Sending local "example" event with one dict as argument
//! altv::events::emit!("example", altv::mvalue::dict!{ "example" => 123 }.unwrap()).unwrap();
//!
//! // Applying a per-resource vehicle meta
//! let vehicle = altv::Vehicle::new("sultan", 0, 0).unwrap();
//! vehicle.set_meta("example", altv::mvalue::dict!{ "example" => 123 }.unwrap()).unwrap();
//! ```
//!
//! How it can be handled in JavaScript:
//! ```js
//! alt.on("example", (dict) => {
//!     alt.log("dict:", dict) // dict: { example: 123 }
//! })
//!
//! // Same for vehicle meta:
//! const vehicle = alt.Vehicle.all[0]
//! const dict = vehicle.getMeta("example")
//! alt.log("dict:", dict) // dict: { example: 123 }
//! ```
pub use core_resource::exports::mvalue::{
    mvalue_dict as dict, mvalue_list as list, MValue, MValueList,
};
