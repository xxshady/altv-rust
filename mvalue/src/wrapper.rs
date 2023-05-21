use crate::types::{RawMValue, RawMutMValue};

use altv_sdk::ffi as sdk;
use autocxx::prelude::*;

pub struct MValue(RawMutMValue);

impl MValue {
    pub fn new(ptr: RawMutMValue) -> Self {
        Self(ptr)
    }

    pub fn get(&self) -> &RawMutMValue {
        &self.0
    }

    pub fn as_mut(&mut self) -> std::pin::Pin<&mut sdk::MValueMutWrapper> {
        self.0.as_mut().unwrap()
    }

    pub fn into_const(self) -> RawMValue {
        unsafe { sdk::convert_mvalue_mut_wrapper_to_const(self.0) }.within_unique_ptr()
    }
}

impl Clone for MValue {
    fn clone(&self) -> Self {
        Self::new(unsafe { sdk::copy_mut_mvalue(self.0.as_ref().unwrap()) }.within_unique_ptr())
    }
}
