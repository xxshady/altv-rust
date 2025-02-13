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

        let base_object_type = altv_sdk::helpers::get_base_object_type($base_object.as_ptr());

        logger::debug!(
          "{} type: {:?}",
          stringified_method_name,
          base_object_type
        );

        let exports = manager.get_resource_exports_by_name($resource_name);
        let Some(exports) = exports else {
          logger::debug!("{} resource: {:?} get_resource_exports_by_name failed", stringified_method_name, $resource_name);
          return;
        };

        exports.[<$method_name>]($base_object, base_object_type)
          // TODO: stop resource on panic if reloading is enabled
          .unwrap();
      });
    }
  };
}
