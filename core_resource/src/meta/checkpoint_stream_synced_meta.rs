use std::rc::Rc;

use crate::{
    base_objects::{checkpoint::CheckpointContainer, ValidBaseObject},
    helpers::IntoString,
    SomeResult,
};

pub struct StreamSyncedCheckpointMetaEntry {
    pub(super) base_object: CheckpointContainer,
    pub(super) key: String,
}

pub trait StreamSyncedCheckpointMeta: ValidBaseObject
where
    CheckpointContainer: From<Rc<Self>>,
{
    /// Provides access to read or modify stream synced meta of Checkpoint.
    ///
    /// # Examples
    /// ```rust
    /// let checkpoint = altv::Checkpoint::new(0, 0, 10.0, 10.0, (255, 255, 255), 10);
    ///
    /// // Set "example" key of stream synced meta to `123`
    /// checkpoint.stream_synced_meta_entry("example")?.set(123)?;
    ///
    /// // Read "example" key of stream synced meta
    /// checkpoint.stream_synced_meta_entry("example")?.get()?; // Some(MValue::I64(123))
    /// ```
    fn stream_synced_meta_entry(
        self: &Rc<Self>,
        key: impl IntoString,
    ) -> SomeResult<StreamSyncedCheckpointMetaEntry> {
        self.assert_valid()?;
        Ok(StreamSyncedCheckpointMetaEntry {
            base_object: self.clone().into(),
            key: key.into_string(),
        })
    }
}
