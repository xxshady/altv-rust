#[macro_export]
macro_rules! __impl_meta {
    (
        $meta_type: ident,
        $sdk_namespace: path
    ) => {
        paste::paste! {
            pub fn [<has_ $meta_type:snake>](key: impl $crate::helpers::IntoString) -> bool {
                unsafe {
                    $sdk_namespace::[<Has $meta_type Data>](key.into_string())
                }
            }

            pub fn [<get_ $meta_type:snake>](
                key: impl $crate::helpers::IntoString,
            ) -> $crate::mvalue::MValue {
                $crate::resource::Resource::with(|resource| {
                    $crate::mvalue::deserialize_from_sdk(
                        unsafe {
                            $sdk_namespace::[<Get $meta_type Data>](key.into_string())
                        },
                        resource,
                    )
                })
            }

            pub fn [<delete_ $meta_type:snake>](key: impl $crate::helpers::IntoString) {
                unsafe {
                    $sdk_namespace::[<Delete $meta_type Data>](key.into_string())
                }
            }

            pub fn [<set_ $meta_type:snake>](
                key: impl $crate::helpers::IntoString,
                value: impl TryInto<$crate::mvalue::Serializable, Error = anyhow::Error>,
            ) -> $crate::VoidResult {
                unsafe {
                    $sdk_namespace::[<Set $meta_type Data>](
                        key.into_string(),
                        value
                            .try_into()
                            .or_else(|e| {
                                anyhow::bail!("Failed to convert value into mvalue, error: {e:?}")
                            })?.0,
                    )
                }
                Ok(())
            }

            pub fn [<get_ $meta_type:snake _keys>]() -> Vec<String> {
                $crate::helpers::read_cpp_str_vec(
                    unsafe { $sdk_namespace::[<Get $meta_type DataKeys>]() }
                )
            }
        }
    };
}

pub use __impl_meta as impl_meta;
