pub fn get_base_object_type(
    base_object: *const altv_sdk::ffi::alt::IBaseObject,
) -> altv_sdk::BaseObjectType {
    let raw_type = unsafe { altv_sdk::ffi::get_base_object_type(base_object) };
    if raw_type == 255 {
        panic!("resource_on_create_base_object base_object type is invalid");
    }

    altv_sdk::BaseObjectType::from(raw_type)
        .expect("failed to convert raw baseobj type to BaseObjectType")
}

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

                if $base_object.is_null() {
                    panic!("{} base_object is null", stringified_method_name);
                }

                let base_object_type = helpers::get_base_object_type($base_object);

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
