use std::marker::PhantomData;

use serde::{de::DeserializeOwned, Serialize};

use crate::{helpers, sdk};

pub struct GlobalMetaEntry<V: Serialize + DeserializeOwned> {
    pub(super) key: String,
    __type: PhantomData<V>,
}

/// Provides access to read or modify global normal meta of alt:V resource.
///
/// # Examples
/// ```rust
/// # mod altv { pub use altv_internal_core_resource::exports::*; }
/// # use altv::meta::MetaEntry;
/// # fn test() -> altv::VoidResult {
/// let example_entry = altv::meta::entry("example");
///
/// // Set "example" key of global normal meta to `123`
/// example_entry.set(&123)?;
///
/// // Read "example" key of global normal meta
/// let value: Option<i32> = example_entry.get()?; // Some(123)
/// # Ok(()) }
/// ```
pub fn entry<V: Serialize + DeserializeOwned>(key: impl ToString) -> GlobalMetaEntry<V> {
    GlobalMetaEntry {
        key: key.to_string(),
        __type: PhantomData,
    }
}

pub fn keys() -> Vec<String> {
    helpers::read_cpp_str_vec(unsafe { sdk::ICore::GetMetaDataKeys() })
}

pub struct GlobalSyncedMetaEntry<V: Serialize + DeserializeOwned> {
    pub(super) key: String,
    __type: PhantomData<V>,
}

/// Provides access to read or modify global **synced** meta of alt:V resource.
///
/// # Examples
/// ```rust
/// # mod altv { pub use altv_internal_core_resource::exports::*; }
/// # use altv::meta::MetaEntry;
/// # fn test() -> altv::VoidResult {
/// let example_entry = altv::meta::synced_entry("example");
///
/// // Set "example" key of global synced meta to `123`
/// example_entry.set(&123)?;
///
/// // Read "example" key of global synced meta
/// let value: Option<i32> = example_entry.get()?; // Some(123)
/// # Ok(()) }
/// ```
pub fn synced_entry<V: Serialize + DeserializeOwned>(
    key: impl ToString,
) -> GlobalSyncedMetaEntry<V> {
    GlobalSyncedMetaEntry {
        key: key.to_string(),
        __type: PhantomData,
    }
}

pub fn synced_keys() -> Vec<String> {
    helpers::read_cpp_str_vec(unsafe { sdk::ICore::GetSyncedMetaDataKeys() })
}
