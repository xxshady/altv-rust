use std::fmt::Debug;

use crate::{
    from_mvalue,
    types::{RawConstMValue, RawMutMValue},
    Result,
};

use altv_sdk::ffi as sdk;
use autocxx::prelude::*;
use serde::de::DeserializeOwned;

pub struct MutMValue(RawMutMValue);

impl MutMValue {
    pub fn new(ptr: RawMutMValue) -> Self {
        Self(ptr)
    }

    pub fn get(&self) -> &RawMutMValue {
        &self.0
    }

    pub fn as_mut(&mut self) -> std::pin::Pin<&mut sdk::MValueMutWrapper> {
        self.0.as_mut().unwrap()
    }

    pub fn into_const(self) -> ConstMValue {
        ConstMValue::new(
            unsafe { sdk::convert_mvalue_mut_wrapper_to_const(self.0) }.within_unique_ptr(),
        )
    }
}

impl Clone for MutMValue {
    fn clone(&self) -> Self {
        Self::new(unsafe { sdk::copy_mut_mvalue(self.0.as_ref().unwrap()) }.within_unique_ptr())
    }
}

impl Debug for MutMValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MutMValue {{ ... }}")
    }
}

pub struct ConstMValue(RawConstMValue);

impl ConstMValue {
    pub fn new(ptr: RawConstMValue) -> Self {
        Self(ptr)
    }

    pub fn get(&self) -> &RawConstMValue {
        &self.0
    }

    pub fn deserialize<V: DeserializeOwned>(&self) -> Result<V> {
        from_mvalue(self)
    }
}

impl Debug for ConstMValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstMValue {{ ... }}")
    }
}

impl From<&sdk::ConstMValueWrapper> for ConstMValue {
    fn from(value: &sdk::ConstMValueWrapper) -> Self {
        Self::new(unsafe { sdk::copy_const_mvalue(value) }.within_unique_ptr())
    }
}

impl Clone for ConstMValue {
    fn clone(&self) -> Self {
        Self::new(unsafe { sdk::copy_const_mvalue(self.0.as_ref().unwrap()) }.within_unique_ptr())
    }
}
