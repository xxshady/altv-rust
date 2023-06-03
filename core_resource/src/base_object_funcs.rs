use altv_sdk::BaseObjectType;

use crate::{base_objects::AnyBaseObject, exports::BaseObjectId, resource::Resource};

pub fn all() -> Vec<AnyBaseObject> {
    Resource::with_base_objects_ref(|v, _| v.all())
}

pub fn all_count() -> usize {
    Resource::with_base_objects_ref(|v, _| v.all_count())
}

pub fn get_by_id(base_object_type: BaseObjectType, id: BaseObjectId) -> Option<AnyBaseObject> {
    Resource::with_base_objects_ref(|v, _| v.get_by_id(base_object_type, id))
}
