use crate::vector::{Vector2, Vector3};
use altv_sdk::ffi as sdk;
use autocxx::prelude::*;

#[macro_export]
macro_rules! get_entity_by_id {
    ($wrapper: path, $entity_id: ident) => {
        $crate::resource::Resource::with_entities_ref(|entities, _| {
            let result = entities.get_by_id($entity_id);

            match result {
                Some(_wrapper @ $wrapper(entity)) => Some(Rc::clone(entity)),
                None | Some(_) => None,
            }
        })
    };
}

pub fn read_cpp_vector3(cpp_vector: UniquePtr<sdk::Vector3Wrapper>) -> Vector3 {
    let mut out_x = 0f32;
    let mut out_y = 0f32;
    let mut out_z = 0f32;
    unsafe {
        sdk::read_vector3(
            cpp_vector.as_ref().unwrap(),
            &mut out_x as *mut f32,
            &mut out_y as *mut f32,
            &mut out_z as *mut f32,
        );
    }
    Vector3::new(out_x, out_y, out_z)
}

pub fn read_cpp_vector2(cpp_vector: UniquePtr<sdk::Vector2Wrapper>) -> Vector2 {
    let mut out_x = 0f32;
    let mut out_y = 0f32;
    unsafe {
        sdk::read_vector2(
            cpp_vector.as_ref().unwrap(),
            &mut out_x as *mut f32,
            &mut out_y as *mut f32,
        );
    }
    Vector2::new(out_x, out_y)
}
