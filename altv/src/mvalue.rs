//! MValue, stands for Multi Value, this is needed so you can tell alt:V
//! how to understand Rust types in more general way
//! to send or receive something from clients (players), or communicate with
//! [other alt:V resources](https://docs.altv.mp/articles/resources.html#types).
//!
//! If you want to know more about MValue in general, you can read more [here](https://docs.altv.mp/sdk/mvalues.html).
//!
//! Rust module uses [`serde`](https://serde.rs) for serializing and deserializing
//! data structures between Rust and alt:V (or other scripting languages, for example JavaScript).
//!
//! For example, alt:V has Dict MValue type, in Rust it can be created
//! (serialized) from `HashMap<String, &dyn Serialize>` or any struct which implements
//! [`Serialize`](https://docs.rs/serde/latest/serde/trait.Serialize.html)
//! ([see](#how-to-implement-serialize-and-deserialize-for-your-struct)).
//!
//! # Supported MValue <-> Rust types
//! [Full list of types](https://github.com/xxshady/altv-rust/blob/577081a3ac0b028e4222584c1b0b79edad3fe405/mvalue/src/helpers.rs#L89-L102)
//!
//! # Examples
//! > *Examples will use [`to_mvalue`](fn.to_mvalue.html) and [`from_mvalue`](fn.from_mvalue.html),
//! these are used internally by events and meta in Rust module.*
//!
//! ### Numbers
//! ```rust
//! # fn test() -> altv::VoidResult {
//! use altv::mvalue::{from_mvalue, to_mvalue};
//!
//! let mvalue = to_mvalue(&123)?;
//! let value: i32 = from_mvalue(&mvalue.into_const())?;
//! dbg!(value); // 123
//!
//! let mvalue = to_mvalue(&256)?;
//! // Will return error and panic because 256 is greater than u8::MAX
//! let value: u8 = from_mvalue(&mvalue.into_const()).unwrap();
//! # Ok(()) }
//! ```
//!
//! ### String, &str, char
//! ```rust
//! # fn test() -> altv::VoidResult {
//! use altv::mvalue::{from_mvalue, to_mvalue};
//!
//! let mvalue = to_mvalue(&String::from("example"))?;
//! let value: String = from_mvalue(&mvalue.into_const())?;
//! dbg!(value); // "example"
//!
//! let mvalue = to_mvalue(&"example")?;
//! // &str can only be deserialized as String
//! let value: String = from_mvalue(&mvalue.into_const())?;
//! dbg!(value); // "example"
//!
//! let mvalue = to_mvalue(&'e')?;
//! let value: char = from_mvalue(&mvalue.into_const())?;
//! dbg!(value); // 'e'
//!
//! let mvalue = to_mvalue(&"eee")?;
//! // Will return error and panic
//! let value: char = from_mvalue(&mvalue.into_const()).unwrap();
//! # Ok(()) }
//! ```
//!
//! ### Option
//! ```rust
//! # fn test() -> altv::VoidResult {
//! use altv::mvalue::{from_mvalue, to_mvalue};
//!
//! let mvalue = to_mvalue(&Some(true))?;
//! let value: Option<bool> = from_mvalue(&mvalue.into_const())?;
//! dbg!(value); // Some(true)
//!
//! // Since there is no Option MValue type, all options containing the value
//! // are "unwrapped" and serialized as-is
//! let mvalue = to_mvalue(&Some(true))?;
//! let value: bool = from_mvalue(&mvalue.into_const())?;
//! dbg!(value); // true
//!
//! let mvalue = to_mvalue(&Option::<bool>::None)?;
//! let value: Option<bool> = from_mvalue(&mvalue.into_const())?;
//! dbg!(value); // None
//! # Ok(()) }
//! ```
//!
//! ### Tuple
//! ```rust
//! # fn test() -> altv::VoidResult {
//! use altv::mvalue::{from_mvalue, to_mvalue};
//!
//! let mvalue = to_mvalue(&(true, 123))?;
//! let value: (bool, i32) = from_mvalue(&mvalue.into_const())?;
//! dbg!(value); // (true, 123)
//!
//! // Tuples can be deserialized partially
//! // (in fact, it works the same for all sequences: arrays, vectors, etc.)
//! let mvalue = to_mvalue(&(true, 123))?;
//! let value: (bool,) = from_mvalue(&mvalue.into_const())?;
//! dbg!(value); // (true,)
//! # Ok(()) }
//! ```
//!
//! ### Byte Array (ByteBuf)
//! ```rust
//! # fn test() -> altv::VoidResult {
//! use altv::mvalue::{from_mvalue, to_mvalue};
//!
//! let mvalue = to_mvalue(&altv::ByteBuf::from([1, 2, 3]))?;
//! let value: altv::ByteBuf = from_mvalue(&mvalue.into_const())?;
//! dbg!(value); // [1, 2, 3]
//! # Ok(()) }
//! ```
//!
//! # How to implement Serialize and Deserialize for your struct
//! ```rust
//! # fn test() -> altv::VoidResult {
//! use altv::serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize, Debug)]
//! #[serde(crate = "altv::serde")]
//! struct MyStruct {
//!     a: i32,
//! }
//!
//! let mvalue = altv::mvalue::to_mvalue(&MyStruct { a: 123 })?;
//! let my_struct: MyStruct = altv::mvalue::from_mvalue(&mvalue.into_const())?;
//! dbg!(my_struct); // MyStruct { a: 123 }
//! # Ok(()) }
//! ```
pub use core_resource::exports::mvalue::{
    from_mvalue, from_mvalue_slice, to_mvalue, ConstMValue, DeserializeMValueArgs, DynMValue,
    DynMValueArgs, Error,
};
