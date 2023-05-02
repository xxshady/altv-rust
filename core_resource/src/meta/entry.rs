use crate::{mvalue, resource::Resource, sdk, VoidResult};

use super::global::{GlobalMetaEntry, GlobalSyncedMetaEntry};

pub trait MetaEntry {
    fn has(&self) -> bool;
    fn get(&self) -> Option<mvalue::MValue>;
    fn set(&self, value: impl TryInto<mvalue::Serializable, Error = anyhow::Error>) -> VoidResult;
    fn delete(&self);
}

macro_rules! impl_global_meta_entry {
    (
        $meta_type: ident,
        $entry_struct: path
    ) => {
        paste::paste! {
            impl MetaEntry for $entry_struct {
                fn has(&self) -> bool {
                    unsafe { sdk::ICore::[<Has $meta_type Data>](&self.key) }
                }

                fn get(&self) -> Option<mvalue::MValue> {
                    let value = Resource::with(|resource| {
                        mvalue::deserialize_from_sdk(unsafe { sdk::ICore::[<Get $meta_type Data>](&self.key) }, resource)
                    });

                    if let mvalue::MValue::None = value {
                        None
                    } else {
                        Some(value)
                    }
                }

                fn set(&self, value: impl TryInto<mvalue::Serializable, Error = anyhow::Error>) -> VoidResult {
                    unsafe {
                        sdk::ICore::[<Set $meta_type Data>](
                            &self.key,
                            value
                                .try_into()
                                .or_else(|e| {
                                    anyhow::bail!("Failed to convert value into mvalue, error: {e:?}")
                                })?
                                .0,
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
