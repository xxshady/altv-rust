use crate::{
    base_object::{BaseObject, BaseObjectManager, BaseObjectPointer},
    base_object_maps::BaseObjectMap,
    impl_base_object_for,
    resource::Resource,
};
use altv_sdk::ffi as sdk;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

#[derive(Debug)]
pub struct VirtualEntityGroup {
    ptr: BaseObjectPointer,
    base_type: altv_sdk::BaseObjectType,
}

impl VirtualEntityGroup {
    pub fn new(max_entities_in_stream: u32) -> VirtualEntityGroupContainer {
        Resource::with_base_object_creation_mut(|_, resource| {
            let base_objects = resource.base_objects.borrow_mut();
            create_virtual_entity_group(base_objects, max_entities_in_stream)
        })
    }

    // cannot be destroyed
    // pub fn destroy(&mut self) -> anyhow::Result<()> {
    //     // TODO: test
    //     // Resource::with_base_object_deletion_mut(|_, resource| {
    //     //     let mut entities = resource.entities.borrow_mut();
    //     //     entities.on_destroy(self.ptr().to_entity().unwrap());
    //     // });

    //     self.destroy_base_object()
    // }

    pub(crate) fn raw_ptr(&self) -> anyhow::Result<*mut sdk::alt::IVirtualEntityGroup> {
        self.ptr().to_virtual_entity_group()
    }

    pub fn id(&self) -> anyhow::Result<u32> {
        Ok(unsafe { sdk::IVirtualEntityGroup::GetID(self.raw_ptr()?) })
    }

    pub fn max_entities_in_stream(&self) -> anyhow::Result<u32> {
        Ok(unsafe { sdk::IVirtualEntityGroup::GetStreamingRangeLimit(self.raw_ptr()?) })
    }
}

impl_base_object_for!(VirtualEntityGroup);

pub type VirtualEntityGroupContainer = Rc<RefCell<VirtualEntityGroup>>;

pub fn create_virtual_entity_group(
    mut base_objects: RefMut<BaseObjectManager>,
    max_entities_in_stream: u32,
) -> VirtualEntityGroupContainer {
    let raw_ptr = unsafe { sdk::ICore::CreateVirtualEntityGroup(max_entities_in_stream) };

    if raw_ptr.is_null() {
        panic!("sdk::ICore::CreateVirtualEntityGroup returned nullptr");
    }

    let base_object_raw_ptr = unsafe { sdk::virtual_entity_group::to_base_object(raw_ptr) };
    let virtual_entity_group: VirtualEntityGroupContainer =
        create_virtual_entity_group_container(base_object_raw_ptr);

    base_objects.on_create(base_object_raw_ptr, virtual_entity_group.clone());

    Resource::with_virtual_entity_group_base_object_map_mut(
        |mut virtual_entity_group_base_object_map, _| {
            virtual_entity_group_base_object_map.add_base_object(Rc::clone(&virtual_entity_group));
        },
    );

    virtual_entity_group
}

pub fn create_virtual_entity_group_container(
    raw_ptr: altv_sdk::IBaseObjectMutPtr,
) -> VirtualEntityGroupContainer {
    Rc::new(RefCell::new(VirtualEntityGroup {
        ptr: BaseObjectPointer::new(raw_ptr),
        base_type: altv_sdk::BaseObjectType::VirtualEntityGroup,
    }))
}
