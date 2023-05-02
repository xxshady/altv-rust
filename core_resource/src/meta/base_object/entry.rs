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
    mvalue,
    resource::Resource,
    sdk, SomeResult, VoidResult,
};

use super::{normal_meta::NormalBaseObjectMetaEntry, synced_meta::SyncedBaseObjectMetaEntry};

pub trait BaseObjectMetaEntry {
    fn has(&self) -> SomeResult<bool>;
    fn get(&self) -> SomeResult<Option<mvalue::MValue>>;
    fn set(&self, value: impl TryInto<mvalue::Serializable, Error = anyhow::Error>) -> VoidResult;
    fn delete(&self) -> VoidResult;
}

macro_rules! impl_base_object_meta_entry {
    (
        $meta_type: ident,
        $entry_struct: path,
        $sdk_namespace: path,
        $raw_ptr: expr,
        $(
            @generics: [ $(
                $generic_param: ident $(: $generic_param_trait: ty )?,
            )+ ]
        )?
    ) => {
        paste::paste! {
            impl $( < $( $generic_param $( : $generic_param_trait )?, )+ > )? BaseObjectMetaEntry for $entry_struct {
                fn has(&self) -> SomeResult<bool> {
                    Ok(unsafe { $sdk_namespace::[<Has $meta_type Data>]($raw_ptr(&self.base_object)?, &self.key) })
                }

                fn get(&self) -> SomeResult<Option<mvalue::MValue>> {
                    let raw_ptr = $raw_ptr(&self.base_object)?;
                    let value = Resource::with(|resource| {
                        mvalue::deserialize_from_sdk(
                            unsafe { $sdk_namespace::[<Get $meta_type Data>](raw_ptr, &self.key) },
                            resource,
                        )
                    });

                    Ok(if let mvalue::MValue::None = value { None } else { Some(value) })
                }

                fn set(&self, value: impl TryInto<mvalue::Serializable, Error = anyhow::Error>) -> VoidResult {
                    unsafe {
                        $sdk_namespace::[<Set $meta_type Data>](
                            $raw_ptr(&self.base_object)?,
                            &self.key,
                            value
                                .try_into()
                                .or_else(|e| {
                                    anyhow::bail!("Failed to convert value to mvalue, error: {e:?}")
                                })?.0,
                        )
                    }
                    Ok(())
                }

                fn delete(&self) -> VoidResult {
                    unsafe { $sdk_namespace::[<Delete $meta_type Data>]($raw_ptr(&self.base_object)?, &self.key) }
                    Ok(())
                }
            }
        }
    };
}

impl_base_object_meta_entry!(
    Meta,
    NormalBaseObjectMetaEntry<T, InheritPtrs>,
    sdk::IBaseObject,
    BaseObjectWrapper::raw_base_ptr,
    @generics: [T, InheritPtrs: Clone,]
);

impl_base_object_meta_entry!(
    SyncedMeta,
    SyncedBaseObjectMetaEntry<T, InheritPtrs>,
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
