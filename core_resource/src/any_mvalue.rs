use std::collections::HashMap;

use serde::Deserialize;
use serde_bytes::ByteBuf;

use crate::{
    base_objects::AnyBaseObject,
    rgba::Rgba,
    vector::{Vector2, Vector3},
};

/// An enum that maps alt:V [SDK MValue](https://docs.altv.mp/sdk/mvalues.html) to Rust types as closely as possible.
///
/// # Examples
/// ```rust
/// # mod altv { pub use altv_internal_core_resource::exports::*; }
/// # fn test() -> altv::VoidResult {
/// altv::events::on("any_mvalue", || {
/// })
/// # Ok(()) }
/// ```
#[derive(Deserialize, Debug)]
pub enum AnyMValue {
    Bool(bool),
    /// Nil or None, see [SDK docs](https://docs.altv.mp/sdk/mvalues.html).
    None,
    Double(f64),
    Int(i64),
    Uint(u64),
    String(String),
    ByteArray(ByteBuf),
    Dict(HashMap<String, AnyMValue>),

    // custom types:
    BaseObject(AnyBaseObject),
    Rgba(Rgba),
    Vector2(Vector2),
    Vector3(Vector3),
}

impl AnyMValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            AnyMValue::Double(v) => Some(*v),
            AnyMValue::Int(v) => Some(*v as _),
            AnyMValue::Uint(v) => Some(*v as _),
            _ => None,
        }
    }
}
