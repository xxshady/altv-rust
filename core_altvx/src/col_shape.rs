use crate::{
    base_object::{BaseObject, BaseObjectManager, BaseObjectPointer},
    impl_base_object_for,
    resource::Resource,
    vector::Vector2,
    world_object::WorldObject,
};
use altv_sdk::ffi as sdk;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

#[derive(Debug)]
pub struct ColShape {
    ptr: BaseObjectPointer,
    base_type: altv_sdk::BaseObjectType,
}

impl ColShape {
    pub fn new_circle(pos: Vector2, radius: f32) -> ColShapeContainer {
        Resource::with_base_object_creation_mut(|_, resource| {
            let base_objects = resource.base_objects.borrow_mut();
            create_col_shape_circle(base_objects, pos, radius)
        })
    }

    pub fn destroy(&mut self) -> anyhow::Result<()> {
        // TODO: test
        // Resource::with_base_object_deletion_mut(|_, resource| {
        //     let mut entities = resource.entities.borrow_mut();
        //     entities.on_destroy(self.ptr().to_entity().unwrap());
        // });

        self.destroy_base_object()
    }
}

impl_base_object_for!(ColShape);
impl WorldObject for ColShape {}

pub type ColShapeContainer = Rc<RefCell<ColShape>>;

pub fn create_col_shape_circle(
    mut base_objects: RefMut<BaseObjectManager>,
    pos: Vector2,
    radius: f32,
) -> ColShapeContainer {
    let raw_ptr = unsafe { sdk::ICore::CreateColShapeCircle(pos.x(), pos.y(), 0.0, radius) };

    if raw_ptr.is_null() {
        panic!("sdk::ICore::CreateColShapeCircle returned nullptr");
    }

    let base_object_raw_ptr = unsafe { sdk::col_shape::to_base_object(raw_ptr) };
    let col_shape: ColShapeContainer = create_col_shape_container(base_object_raw_ptr);

    base_objects.on_create(base_object_raw_ptr, col_shape.clone());

    Resource::with_col_shape_base_object_map_mut(|mut col_shape_base_object_map, _| {
        col_shape_base_object_map.add_base_object(Rc::clone(&col_shape));
    });

    col_shape
}

pub fn create_col_shape_container(raw_ptr: altv_sdk::IBaseObjectMutPtr) -> ColShapeContainer {
    Rc::new(RefCell::new(ColShape {
        ptr: BaseObjectPointer::new(raw_ptr),
        base_type: altv_sdk::BaseObjectType::Colshape,
    }))
}
