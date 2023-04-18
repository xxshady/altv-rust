use crate::{helpers::IntoString, mvalue, SomeResult, VoidResult};

pub trait Meta {
    fn has_meta(&self, key: impl IntoString) -> SomeResult<bool>;
    fn get_meta(&self, key: impl IntoString) -> SomeResult<mvalue::MValue>;
    fn delete_meta(&self, key: impl IntoString) -> VoidResult;
    fn set_meta(
        &self,
        key: impl IntoString,
        value: impl TryInto<mvalue::Serializable, Error = anyhow::Error>,
    ) -> VoidResult;
    fn get_meta_keys(&self) -> SomeResult<Vec<String>>;
}

pub use Meta as BaseObjectMeta;

pub trait StreamSyncedMeta {
    fn has_stream_synced_meta(&self, key: impl IntoString) -> SomeResult<bool>;
    fn get_stream_synced_meta(&self, key: impl IntoString) -> SomeResult<mvalue::MValue>;
    fn delete_stream_synced_meta(&self, key: impl IntoString) -> VoidResult;
    fn set_stream_synced_meta(
        &self,
        key: impl IntoString,
        value: impl TryInto<mvalue::Serializable, Error = anyhow::Error>,
    ) -> VoidResult;
    fn get_stream_synced_meta_keys(&self) -> SomeResult<Vec<String>>;
}

pub trait SyncedMeta {
    fn has_synced_meta(&self, key: impl IntoString) -> SomeResult<bool>;
    fn get_synced_meta(&self, key: impl IntoString) -> SomeResult<mvalue::MValue>;
    fn delete_synced_meta(&self, key: impl IntoString) -> VoidResult;
    fn set_synced_meta(
        &self,
        key: impl IntoString,
        value: impl TryInto<mvalue::Serializable, Error = anyhow::Error>,
    ) -> VoidResult;
    fn get_synced_meta_keys(&self) -> SomeResult<Vec<String>>;
}

pub trait LocalMeta {
    fn has_local_meta(&self, key: impl IntoString) -> SomeResult<bool>;
    fn get_local_meta(&self, key: impl IntoString) -> SomeResult<mvalue::MValue>;
    fn delete_local_meta(&self, key: impl IntoString) -> VoidResult;
    fn set_local_meta(
        &self,
        key: impl IntoString,
        value: impl TryInto<mvalue::Serializable, Error = anyhow::Error>,
    ) -> VoidResult;
    fn get_local_meta_keys(&self) -> SomeResult<Vec<String>>;
}

#[macro_export]
macro_rules! __impl_meta_type_for {
    (
        $meta_type: ident,
        $entity: path,
        $sdk_namespace: path,
        $raw_ptr: expr
        $(, $generic_param: ident )?
    ) => {
        paste::paste! {
            impl $( < $generic_param > )? $crate::base_objects::meta::[<$meta_type>] for $entity {
                fn [<has_ $meta_type:snake>](&self, key: impl $crate::helpers::IntoString) -> SomeResult<bool> {
                    Ok(unsafe {
                        $sdk_namespace::[<Has $meta_type Data>]($raw_ptr(self)?, key.into_string())
                    })
                }

                fn [<get_ $meta_type:snake>](
                    &self,
                    key: impl $crate::helpers::IntoString,
                ) -> SomeResult<$crate::mvalue::MValue> {
                    Resource::with(|v| {
                        Ok($crate::mvalue::deserialize_from_sdk(
                            unsafe {
                                $sdk_namespace::[<Get $meta_type Data>]($raw_ptr(self)?, key.into_string())
                            },
                            v,
                        ))
                    })
                }

                fn [<delete_ $meta_type:snake>](&self, key: impl $crate::helpers::IntoString) -> VoidResult {
                    unsafe {
                        $sdk_namespace::[<Delete $meta_type Data>]($raw_ptr(self)?, key.into_string())
                    }
                    Ok(())
                }

                fn [<set_ $meta_type:snake>](
                    &self,
                    key: impl $crate::helpers::IntoString,
                    value: impl TryInto<$crate::mvalue::Serializable, Error = anyhow::Error>,
                ) -> VoidResult {
                    unsafe {
                        $sdk_namespace::[<Set $meta_type Data>](
                            $raw_ptr(self)?,
                            key.into_string(),
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

                fn [<get_ $meta_type:snake _keys>](&self) -> SomeResult<Vec<String>> {
                    Ok(
                        unsafe { $sdk_namespace::[<Get $meta_type DataKeys>]($raw_ptr(self)?) }
                            .into_iter()
                            .map(|v| v.to_string())
                            .collect(),
                    )
                }
            }
        }
    };
}

pub use __impl_meta_type_for as impl_meta_type_for;

#[macro_export]
macro_rules! __impl_entity_meta_for {
    ($meta_type: ident, $entity: path) => {
        $crate::base_objects::meta::impl_meta_type_for!(
            $meta_type,
            $entity,
            $crate::sdk::IEntity,
            Entity::raw_ptr
        );
    };
}

pub use __impl_entity_meta_for as impl_entity_meta_for;
