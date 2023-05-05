use crate::{base_objects::AnyBaseObject, resource::Resource};

pub fn all() -> Vec<AnyBaseObject> {
    Resource::with_base_objects_ref(|v, _| v.all())
}

pub fn all_count() -> usize {
    Resource::with_base_objects_ref(|v, _| v.all_count())
}
