use autocxx::prelude::*;
use serde::{de::DeserializeOwned, Serialize};

use super::global::{GlobalMetaEntry, GlobalSyncedMetaEntry};
use crate::{sdk, SomeResult, VoidResult};

pub trait MetaEntry<V: Serialize + DeserializeOwned> {
    fn has(&self) -> bool;
    fn get(&self) -> SomeResult<Option<V>>;

    /// Ensures a value is in the entry by setting provided `value` if empty,
    /// and returns the value in the entry.<br>
    /// Similar to [`or_insert`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert) of HashMap.
    fn get_or_set(&self, value: V) -> SomeResult<V>;

    fn set(&self, value: &V) -> VoidResult;
    fn delete(&self);
}

macro_rules! impl_global_meta_entry {
    (
        $meta_type:ident,
        $entry_struct:path
    ) => {
        paste::paste! {
            impl<V: Serialize + DeserializeOwned> MetaEntry<V> for $entry_struct<V> {
                fn has(&self) -> bool {
                    unsafe { sdk::ICore::[<Has $meta_type Data>](&self.key) }
                }

                fn get(&self) -> SomeResult<Option<V>> {
                    let mvalue = unsafe { sdk::ICore::[<Get $meta_type Data>](&self.key) }.within_unique_ptr();
                    let mvalue = mvalue::ConstMValue::new(mvalue);
                    let deserialized: Option<V> = mvalue::from_mvalue(&mvalue)?;
                    Ok(deserialized)
                }

                fn get_or_set(&self, value: V) -> SomeResult<V> {
                    let current_value: Option<V> = self.get()?;
                    if let Some(v) = current_value {
                        Ok(v)
                    } else {
                        self.set(&value)?;
                        return Ok(value);
                    }
                }

                fn set(&self, value: &V) -> VoidResult {
                    unsafe {
                        sdk::ICore::[<Set $meta_type Data>](
                            &self.key,
                            mvalue::to_mvalue(value)?.get(),
                        )
                    }
                    Ok(())
                }

                fn delete(&self) {
                    unsafe { sdk::ICore::[<Delete $meta_type Data>](&self.key) }
                }
            }
        }
    };
}

impl_global_meta_entry!(Meta, GlobalMetaEntry);
impl_global_meta_entry!(SyncedMeta, GlobalSyncedMetaEntry);
