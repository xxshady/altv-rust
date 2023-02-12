use crate::{
    base_object::{
        BaseObject, BaseObjectPointer, PendingBaseObjectCreation, RawBaseObjectPointer,
        BASE_OBJECT_CREATION_INSTANCE, BASE_OBJECT_MANAGER_INSTANCE,
    },
    entity::{Entity, EntityId, EntityWrapper, ENTITY_MANAGER_INSTANCE},
    vector::Vector3,
};
use altv_sdk::ffi as sdk;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Debug)]
pub struct Vehicle {
    ptr: BaseObjectPointer,
    base_type: altv_sdk::BaseObjectType,
}

impl Vehicle {
    pub fn new(model: u32, pos: Vector3, rot: Vector3) -> Option<VehicleContainer> {
        create_vehicle(
            BASE_OBJECT_CREATION_INSTANCE
                .get()
                .unwrap()
                .try_lock()
                .unwrap(),
            model,
            pos,
            rot,
        )
    }

    pub fn get_by_id(id: EntityId) -> Option<VehicleContainer> {
        let manager = ENTITY_MANAGER_INSTANCE.get().unwrap().try_lock().unwrap();
        let result = manager.get_by_id(id);

        dbg!(result);

        match result {
            Some(_wrapper @ EntityWrapper::Vehicle(vehicle)) => Some(vehicle.clone()),
            None | Some(_) => None,
        }
    }

    pub fn set_secondary_color(&self, color: u8) -> Result<(), String> {
        unsafe { sdk::set_vehicle_primary_color(self.ptr.to_vehicle()?, color) };
        Ok(())
    }

    pub fn get_secondary_color(&self) -> Result<u8, String> {
        Ok(unsafe { sdk::get_vehicle_primary_color(self.ptr.to_vehicle()?) })
    }

    pub fn destroy(&mut self) -> Result<(), String> {
        ENTITY_MANAGER_INSTANCE
            .get()
            .unwrap()
            .try_lock()
            .unwrap()
            .on_destroy(self.ptr().to_entity().unwrap());

        self.destroy_base_object()
    }
}

impl BaseObject for Vehicle {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn ptr(&self) -> &BaseObjectPointer {
        &self.ptr
    }

    fn ptr_mut(&mut self) -> &mut BaseObjectPointer {
        &mut self.ptr
    }

    fn base_type(&self) -> altv_sdk::BaseObjectType {
        self.base_type
    }
}
impl Entity for Vehicle {}

pub type VehicleContainer = Arc<Mutex<Vehicle>>;

pub fn create_vehicle(
    _: MutexGuard<PendingBaseObjectCreation>,
    model: u32,
    pos: Vector3,
    rot: Vector3,
) -> Option<VehicleContainer> {
    let raw_ptr =
        unsafe { sdk::create_vehicle(model, pos.x(), pos.y(), pos.z(), rot.x(), rot.y(), rot.z()) };

    if raw_ptr.is_null() {
        return None;
    }

    let base_object_raw_ptr = unsafe { sdk::convert_vehicle_to_baseobject(raw_ptr) };
    let vehicle: VehicleContainer = create_vehicle_container(base_object_raw_ptr);

    BASE_OBJECT_MANAGER_INSTANCE
        .get()
        .unwrap()
        .try_lock()
        .unwrap()
        .on_create(base_object_raw_ptr, vehicle.clone());

    ENTITY_MANAGER_INSTANCE
        .get()
        .unwrap()
        .try_lock()
        .unwrap()
        .on_create(
            vehicle.try_lock().unwrap().id().unwrap(),
            EntityWrapper::Vehicle(vehicle.clone()),
        );

    Some(vehicle)
}

pub fn create_vehicle_container(raw_ptr: RawBaseObjectPointer) -> VehicleContainer {
    Arc::new(Mutex::new(Vehicle {
        ptr: BaseObjectPointer::new(raw_ptr),
        base_type: altv_sdk::BaseObjectType::VEHICLE,
    }))
}
