use std::collections::HashMap;

use anyhow::bail;
use core_shared::result::SomeResult;
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
///
/// Log content of any mvalue.
/// ```rust
/// # mod altv { pub use altv_internal_core_resource::exports::*; }
/// # fn test() -> altv::VoidResult {
/// altv::events::on("any_mvalue", |event| {
///     // will only fail
///     let (any,): (altv::mvalue::AnyMValue,) = event.args.deserialize()?;
///     dbg!(any);
/// });
/// # Ok(()) }
/// ```
///
/// Receive number (double or integer) from JS
/// (it won't work for bigint though, bigint is serialized as `AnyMValue::UInt`).
/// ```rust
/// # mod altv { pub use altv_internal_core_resource::exports::*; }
/// # fn test() -> altv::VoidResult {
/// altv::events::on("any_mvalue", |event| {
///     let (js_number,): (altv::mvalue::AnyMValue,) = event.args.deserialize()?;
///     let num: f64 = js_number.as_f64()?;
/// });
/// # Ok(()) }
/// ```
#[derive(Debug, Deserialize)]
#[serde(rename = "___altv_any_enum_mvalue")]
pub enum AnyMValue {
    None,
    Nil,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Double(f64),
    String(String),
    List(Vec<AnyMValue>),
    Dict(HashMap<String, AnyMValue>),
    BaseObject(AnyBaseObject),
    /// Not supported (yet?)
    Function,
    Vector3(Vector3),
    Rgba(Rgba),
    ByteArray(ByteBuf),
    Vector2(Vector2),
}

impl AnyMValue {
    pub fn as_f64(&self) -> SomeResult<f64> {
        match self {
            AnyMValue::Double(v) => Ok(*v),
            AnyMValue::Int(v) => Ok(*v as _),
            _ => bail!("Expected AnyMValue::Double or AnyMValue::Int, received: {self:?}"),
        }
    }
}
