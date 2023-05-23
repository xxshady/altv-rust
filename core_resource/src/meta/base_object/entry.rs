use serde::{de::DeserializeOwned, Serialize};

use autocxx::prelude::*;

use crate::{
    base_objects::{
        checkpoint, extra_pools::AnyEntity, player, virtual_entity, BaseObjectWrapper, BasePtr,
    },
    meta::{
        checkpoint_stream_synced_meta::StreamSyncedCheckpointMetaEntry,
        entity_stream_synced_meta::StreamSyncedEntityMetaEntry,
        player_local_meta::LocalPlayerMetaEntry,
        ve_stream_synced_meta::StreamSyncedVirtualEntityMetaEntry,
    },
    sdk, SomeResult, VoidResult,
};

use super::{normal_meta::NormalBaseObjectMetaEntry, synced_meta::SyncedBaseObjectMetaEntry};

pub trait BaseObjectMetaEntry<V: Serialize + DeserializeOwned> {
    fn has(&self) -> SomeResult<bool>;
    fn get(&self) -> SomeResult<Option<V>>;

    /// Ensures a value is in the entry by setting provided `value` if empty,
    /// and returns the value in the entry.<br>
    /// Similar to [`or_insert`](https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_insert) of HashMap.
    ///
    /// # Examples
    /// ```rust
    /// # mod altv { pub use altv_internal_core_resource::exports::*; }
    /// # use altv::meta::{BaseObjectMetaEntry, StreamSyncedEntityMeta};
    /// # fn test() -> altv::VoidResult {
    /// let vehicle = altv::Vehicle::new("sultan2", 0, 0)?;
    ///
    /// let already_set_entry = vehicle.stream_synced_meta_entry("already_set")?;
    ///
    /// already_set_entry.set(&228)?;
    ///
    /// // Returns `228` because entry already contained it
    /// let value = already_set_entry.get_or_set(1337)?;
    /// assert_eq!(value, 228);
    ///
    /// let empty_entry = vehicle.stream_synced_meta_entry("empty")?;
    ///
    /// // Returns `1337` because entry was empty
    /// let value = empty_entry.get_or_set(1337)?;
    /// assert_eq!(value, 1337);
    ///
    /// // Returns `Some(1337)`
    /// let value: Option<i32> = empty_entry.get()?;
    /// assert_eq!(value, Some(1337));
    /// # Ok(()) }
    /// ```
    fn get_or_set(&self, value: V) -> SomeResult<V>;

    fn set(&self, value: &V) -> VoidResult;
    fn delete(&self) -> VoidResult;
}

macro_rules! impl_base_object_meta_entry {
    (
        $meta_type:ident,
        $entry_struct:path,
        $sdk_namespace:path,
        $raw_ptr:expr,
        $(
            @generics: [ $(
                $generic_param:ident $(: $generic_param_trait:ty )?,
            )+ ]
        )?
    ) => {
        paste::paste! {
            impl <
                V: Serialize + DeserializeOwned,
                $( $( $generic_param $( : $generic_param_trait )?, )+ )?
            > BaseObjectMetaEntry<V> for $entry_struct<
                V,
                $( $( $generic_param, )+ )?
            > {
                fn has(&self) -> SomeResult<bool> {
                    Ok(unsafe { $sdk_namespace::[<Has $meta_type Data>]($raw_ptr(&self.base_object)?, &self.key) })
                }

                fn get(&self) -> SomeResult<Option<V>> {
                    let raw_ptr = $raw_ptr(&self.base_object)?;

                    let mvalue = unsafe {
                        $sdk_namespace::[<Get $meta_type Data>](raw_ptr, &self.key)
                    }.within_unique_ptr();
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
                        $sdk_namespace::[<Set $meta_type Data>](
                            $raw_ptr(&self.base_object)?,
                            &self.key,
                            mvalue::to_mvalue(value)?.get(),
                        )
                    }
                    Ok(())
                }

                fn delete(&self) -> VoidResult {
                    unsafe {
                        $sdk_namespace::[<Delete $meta_type Data>](
                            $raw_ptr(&self.base_object)?,
                            &self.key
                        )
                    }
                    Ok(())
                }
            }
        }
    };
}

impl_base_object_meta_entry!(
    Meta,
    NormalBaseObjectMetaEntry,
    sdk::IBaseObject,
    BaseObjectWrapper::raw_base_ptr,
    @generics: [T, InheritPtrs: Clone,]
);

impl_base_object_meta_entry!(
    SyncedMeta,
    SyncedBaseObjectMetaEntry,
    sdk::IBaseObject,
    BaseObjectWrapper::raw_base_ptr,
    @generics: [T, InheritPtrs: Clone,]
);

impl_base_object_meta_entry!(
    StreamSyncedMeta,
    StreamSyncedEntityMetaEntry,
    sdk::IEntity,
    AnyEntity::raw_ptr,
);

impl_base_object_meta_entry!(
    StreamSyncedMeta,
    StreamSyncedCheckpointMetaEntry,
    sdk::ICheckpoint,
    checkpoint::Checkpoint::raw_ptr,
);

impl_base_object_meta_entry!(
    LocalMeta,
    LocalPlayerMetaEntry,
    sdk::IPlayer,
    player::Player::raw_ptr,
);

impl_base_object_meta_entry!(
    StreamSyncedMeta,
    StreamSyncedVirtualEntityMetaEntry,
    sdk::IVirtualEntity,
    virtual_entity::VirtualEntity::raw_ptr,
);
