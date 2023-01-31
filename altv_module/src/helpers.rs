pub fn get_base_object_type(
    base_object: *const altv_sdk::ffi::IBaseObject,
) -> altv_sdk::BaseObjectType {
    let raw_type = unsafe { altv_sdk::ffi::get_baseobject_type(base_object) };
    if raw_type == 255 {
        panic!("resource_on_create_base_object base_object type is invalid");
    }

    altv_sdk::BaseObjectType::from(raw_type)
        .expect("failed to convert raw baseobj type to BaseObjectType")
}
