use crate::{
    base_object::{BaseObject, BaseObjectManager, BaseObjectPointer},
    base_object_maps::BaseObjectMap,
    entity::{Entity, EntityId, EntityWrapper},
    impl_base_object_for,
    resource::Resource,
    vector::Vector3,
    world_object::WorldObject,
};
use altv_sdk::ffi as sdk;
use std::{
    cell::{RefCell, RefMut},
    rc::Rc,
};

#[derive(Debug)]
pub struct Vehicle {
    ptr: BaseObjectPointer,
    base_type: altv_sdk::BaseObjectType,
}

impl Vehicle {
    pub fn new(model: u32, pos: Vector3, rot: Vector3) -> Option<VehicleContainer> {
        Resource::with_base_object_creation_mut(|_, resource| {
            let base_objects = resource.base_objects.borrow_mut();
            create_vehicle(base_objects, model, pos, rot)
        })
    }

    pub fn get_by_id(id: EntityId) -> Option<VehicleContainer> {
        crate::get_entity_by_id!(EntityWrapper::Vehicle, id)
    }

    pub fn set_secondary_color(&self, color: u8) -> anyhow::Result<()> {
        unsafe { sdk::IVehicle::SetPrimaryColor(self.ptr.to_vehicle()?, color) };
        Ok(())
    }

    pub fn get_secondary_color(&self) -> anyhow::Result<u8> {
        Ok(unsafe { sdk::IVehicle::GetPrimaryColor(self.ptr.to_vehicle()?) })
    }

    pub fn destroy(&mut self) -> anyhow::Result<()> {
        Resource::with_base_object_deletion_mut(|_, resource| {
            let mut entities = resource.entities.borrow_mut();
            entities.on_destroy(self.ptr().to_entity().unwrap());
        });

        self.destroy_base_object()
    }
}

impl_base_object_for!(Vehicle);
impl WorldObject for Vehicle {}
impl Entity for Vehicle {}

pub type VehicleContainer = Rc<RefCell<Vehicle>>;

pub fn create_vehicle(
    mut base_objects: RefMut<BaseObjectManager>,
    model: u32,
    pos: Vector3,
    rot: Vector3,
) -> Option<VehicleContainer> {
    let raw_ptr = unsafe {
        sdk::ICore::CreateVehicle(model, pos.x(), pos.y(), pos.z(), rot.x(), rot.y(), rot.z())
    };

    if raw_ptr.is_null() {
        return None;
    }

    let base_object_raw_ptr = unsafe { sdk::vehicle::to_base_object(raw_ptr) };
    let vehicle: VehicleContainer = create_vehicle_container(base_object_raw_ptr);

    base_objects.on_create(base_object_raw_ptr, vehicle.clone());

    Resource::with_entities_mut(|mut entities, resource| {
        entities.on_create(
            vehicle.borrow().id().unwrap(),
            EntityWrapper::Vehicle(Rc::clone(&vehicle)),
        );

        let mut vehicle_base_object_map = resource.vehicle_base_object_map.borrow_mut();
        vehicle_base_object_map.add_base_object(Rc::clone(&vehicle));
    });

    Some(vehicle)
}

pub fn create_vehicle_container(raw_ptr: altv_sdk::IBaseObjectMutPtr) -> VehicleContainer {
    Rc::new(RefCell::new(Vehicle {
        ptr: BaseObjectPointer::new(raw_ptr),
        base_type: altv_sdk::BaseObjectType::Vehicle,
    }))
}
