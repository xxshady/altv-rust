use std::collections::HashMap;

use autocxx::prelude::*;
use crate::{sdk, SomeResult};

pub type HashMapSerialize<'a> = HashMap<String, &'a dyn erased_serde::Serialize>;

pub struct MValueHashMap<'a>(HashMapSerialize<'a>);

impl<'a> MValueHashMap<'a> {
    pub fn new(hash_map: HashMapSerialize<'a>) -> Self {
        Self(hash_map)
    }

    pub(crate) fn to_cpp(self) -> SomeResult<UniquePtr<sdk::MValueUnorderedMapWrapper>> {
        let mut mvalue_map = unsafe { sdk::create_mvalue_unordered_map() }.within_unique_ptr();
        for (key, value) in self.0 {
            unsafe {
                sdk::push_to_mvalue_unordered_map(
                    mvalue_map.as_mut().unwrap(),
                    key.to_string(),
                    mvalue::to_mvalue(value)?.get(),
                )
            }
        }
        Ok(mvalue_map)
    }
}

impl Default for MValueHashMap<'_> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}
