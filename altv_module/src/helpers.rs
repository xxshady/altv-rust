#[macro_export]
macro_rules! on_base_object_event {
    ($method_name:ident, $resource_name:expr, $base_object:expr) => {
        paste::paste! {
            RESOURCE_MANAGER_INSTANCE.with(|manager| {
                let stringified_method_name = stringify!([$method_name]);

                let manager = manager.borrow();
                if manager.is_pending(&$resource_name) {
                    logger::debug!(
                        "{} resource start is pending: {}",
                        stringified_method_name,
                        $resource_name
                    );
                    return;
                }

                let base_object_type = unsafe { altv_sdk::helpers::get_base_object_type($base_object.as_ptr()) };

                logger::debug!(
                    "{} type: {:?}",
                    stringified_method_name,
                    base_object_type
                );

                manager
                    .get_resource_for_module_by_name($resource_name)
                    .unwrap_or_else(|| {
                        panic!("{} resource: {:?} get_resource_for_module_by_path failed", stringified_method_name, $resource_name);
                    })
                    .[<$method_name>]($base_object, base_object_type);
            });
        }
    };
}
