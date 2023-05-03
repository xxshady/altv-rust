use crate::{
    helpers::{self, IntoString},
    sdk,
};

pub struct GlobalMetaEntry {
    pub(super) key: String,
}

/// Provides access to read or modify global meta of alt:V resource.
///
/// # Examples
/// ```rust
/// // Set "example" key of global meta to `123`
/// altv::meta::entry("example").set(123)?;
///
/// // Read "example" key of global meta
/// altv::meta::entry("example").get(); // Some(MValue::I64(123))
/// ```
pub fn entry(key: impl IntoString) -> GlobalMetaEntry {
    GlobalMetaEntry {
        key: key.into_string(),
    }
}

pub fn keys() -> Vec<String> {
    helpers::read_cpp_str_vec(unsafe { sdk::ICore::GetMetaDataKeys() })
}

pub struct GlobalSyncedMetaEntry {
    pub(super) key: String,
}

/// Provides access to read or modify global **synced** meta of alt:V resource.
///
/// # Examples
/// ```rust
/// // Set "example" key of global synced meta to `123`
/// altv::meta::synced_entry("example").set(123)?;
///
/// // Read "example" key of global synced meta
/// altv::meta::synced_entry("example").get(); // Some(MValue::I64(123))
/// ```
pub fn synced_entry(key: impl IntoString) -> GlobalSyncedMetaEntry {
    GlobalSyncedMetaEntry {
        key: key.into_string(),
    }
}

pub fn synced_keys() -> Vec<String> {
    helpers::read_cpp_str_vec(unsafe { sdk::ICore::GetSyncedMetaDataKeys() })
}
