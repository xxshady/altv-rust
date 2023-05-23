use std::{marker::PhantomData, rc::Rc};

use serde::{de::DeserializeOwned, Serialize};

use crate::{
    base_objects::{checkpoint::CheckpointRc, ValidBaseObject},
    helpers::{self, IntoString},
    sdk, SomeResult,
};

pub struct StreamSyncedCheckpointMetaEntry<V> {
    pub(super) base_object: CheckpointRc,
    pub(super) key: String,
    __type: PhantomData<V>,
}

pub trait StreamSyncedCheckpointMeta: ValidBaseObject
where
    CheckpointRc: From<Rc<Self>>,
{
    /// Provides access to read or modify **stream synced** meta of Checkpoint.
    ///
    /// # Examples
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::meta::{BaseObjectMetaEntry, StreamSyncedCheckpointMeta};
    /// # fn test() -> altv::VoidResult {
    /// let checkpoint = altv::Checkpoint::new(0, 0, 10.0, 10.0, (255, 255, 255), 10);
    ///
    /// let example_entry = checkpoint.stream_synced_meta_entry("example")?;
    ///
    /// // Set "example" key of stream synced meta to `123`
    /// example_entry.set(&123)?;
    ///
    /// // Read "example" key of stream synced meta
    /// let value: Option<i32> = example_entry.get()?; // Some(123)
    /// # Ok(()) }
    /// ```
    fn stream_synced_meta_entry<V: Serialize + DeserializeOwned>(
        self: &Rc<Self>,
        key: impl IntoString,
    ) -> SomeResult<StreamSyncedCheckpointMetaEntry<V>> {
        self.assert_valid()?;
        let checkpoint: CheckpointRc = self.clone().into();
        Ok(StreamSyncedCheckpointMetaEntry {
            base_object: checkpoint,
            key: key.into_string(),
            __type: PhantomData,
        })
    }

    fn stream_synced_meta_keys(self: &Rc<Self>) -> SomeResult<Vec<String>> {
        let checkpoint: CheckpointRc = self.clone().into();
        Ok(helpers::read_cpp_str_vec(unsafe {
            sdk::ICheckpoint::GetStreamSyncedMetaDataKeys(checkpoint.raw_ptr()?)
        }))
    }
}
