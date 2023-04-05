#[macro_export]
macro_rules! on_base_object_event {
    ($method_name: ident, $full_main_path: expr, $base_object: expr) => {
        paste::paste! {
            RESOURCE_MANAGER_INSTANCE.with(|manager| {
                let stringified_method_name = stringify!([$method_name]);

                let manager = manager.borrow();
                if manager.is_pending(&$full_main_path) {
                    logger::debug!(
                        "{} resource start is pending: {}",
                        stringified_method_name,
                        $full_main_path
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
                    .get_resource_for_module_by_path($full_main_path)
                    .unwrap_or_else(|| {
                        panic!("{} resource: {:?} get_resource_for_module_by_path failed", stringified_method_name, $full_main_path);
                    })
                    .[<$method_name>]($base_object, base_object_type);
            });
        }
    };
}
