#[macro_export]
macro_rules! base_object_wrapper_impl_link {
    ($manager_name: ident, ()) => {
        concat!(
            "[Implementation](struct.BaseObjectWrapper.html#impl-BaseObjectWrapper<I",
            stringify!($manager_name),
            ",+()>)"
        )
    };
    ($manager_name: ident, ( $not_empty_type: ty )) => {
        concat!(
            "[Implementation](struct.BaseObjectWrapper.html#impl-BaseObjectWrapper<I",
            stringify!($manager_name),
            ",+",
            stringify!($not_empty_type),
            ">)"
        )
    };
}
pub use base_object_wrapper_impl_link;
