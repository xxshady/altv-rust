use crate::{
    base_object::{BaseObject, BaseObjectManager, BaseObjectPointer},
    base_object_maps::{self, BaseObjectMap},
    impl_base_object_for,
    resource::Resource,
    vector::Vector3,
    virtual_entity_group::VirtualEntityGroupContainer,
    world_object::WorldObject,
};
use altv_sdk::ffi as sdk;
use std::{
    cell::{RefCell, RefMut},
    fmt::Debug,
    rc::Rc,
};

#[derive(Debug)]
pub struct VirtualEntity {
    ptr: BaseObjectPointer,
    base_type: altv_sdk::BaseObjectType,
}

impl VirtualEntity {
    pub fn new(
        group: VirtualEntityGroupContainer,
        pos: Vector3,
        streaming_distance: u32,
    ) -> anyhow::Result<VirtualEntityContainer> {
        Resource::with_base_object_creation_mut(|_, resource| {
            let base_objects = resource.base_objects.borrow_mut();
            create_virtual_entity(base_objects, group, pos, streaming_distance)
        })
    }

    pub fn group(&self) -> anyhow::Result<VirtualEntityGroupContainer> {
        let group = unsafe { sdk::IVirtualEntity::GetGroup(self.ptr().to_virtual_entity()?) };
        Resource::with(|r| {
            base_object_maps::get_virtual_entity_group!(
                unsafe { sdk::virtual_entity_group::to_base_object(group) },
                r
            )
        })
        .ok_or(anyhow::anyhow!(
            "failed to get virtual entity group from base object map",
        ))
    }

    pub fn id(&self) -> anyhow::Result<u32> {
        Ok(unsafe { sdk::IVirtualEntity::GetID(self.ptr().to_virtual_entity()?) })
    }

    pub fn streaming_distance(&self) -> anyhow::Result<u32> {
        Ok(unsafe { sdk::IVirtualEntity::GetStreamingDistance(self.ptr().to_virtual_entity()?) })
    }

    pub fn destroy(&mut self) -> anyhow::Result<()> {
        // TODO: test
        // Resource::with_base_object_deletion_mut(|_, resource| {
        //     let mut entities = resource.entities.borrow_mut();
        //     entities.on_destroy(self.ptr().to_entity().unwrap());
        // });

        self.destroy_base_object()
    }

    pub(crate) fn raw_ptr(&self) -> anyhow::Result<*mut sdk::alt::IVirtualEntity> {
        self.ptr().to_virtual_entity()
    }
}

impl_base_object_for!(VirtualEntity);
impl WorldObject for VirtualEntity {}

pub type VirtualEntityContainer = Rc<RefCell<VirtualEntity>>;

pub fn create_virtual_entity(
    mut base_objects: RefMut<BaseObjectManager>,
    group: VirtualEntityGroupContainer,
    pos: Vector3,
    streaming_distance: u32,
) -> anyhow::Result<VirtualEntityContainer> {
    let raw_ptr = unsafe {
        sdk::ICore::CreateVirtualEntity(
            group.try_borrow()?.raw_ptr()?,
            pos.x(),
            pos.y(),
            pos.z(),
            streaming_distance,
        )
    };

    if raw_ptr.is_null() {
        panic!("sdk::ICore::CreateVirtualEntity returned nullptr");
    }

    let base_object_raw_ptr = unsafe { sdk::virtual_entity::to_base_object(raw_ptr) };
    let virtual_entity: VirtualEntityContainer =
        create_virtual_entity_container(base_object_raw_ptr);

    base_objects.on_create(base_object_raw_ptr, virtual_entity.clone());

    Resource::with_virtual_entity_base_object_map_mut(|mut virtual_entity_base_object_map, _| {
        virtual_entity_base_object_map.add_base_object(Rc::clone(&virtual_entity));
    });

    Ok(virtual_entity)
}

pub fn create_virtual_entity_container(
    raw_ptr: altv_sdk::IBaseObjectMutPtr,
) -> VirtualEntityContainer {
    Rc::new(RefCell::new(VirtualEntity {
        ptr: BaseObjectPointer::new(raw_ptr),
        base_type: altv_sdk::BaseObjectType::VirtualEntity,
    }))
}
