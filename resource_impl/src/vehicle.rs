use crate::{
    base_object::{BaseObject, BaseObjectManager, BaseObjectPointer, RawBaseObjectPointer},
    entity::{Entity, EntityId, EntityWrapper},
    resource_impl::RESOURCE_IMPL_INSTANCE,
    vector::Vector3,
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
        RESOURCE_IMPL_INSTANCE.with(|instance| {
            let instance = instance.borrow();
            let _creation = instance.borrow_mut_base_object_creation();
            let base_objects = instance.borrow_mut_base_objects();
            create_vehicle(base_objects, model, pos, rot)
        })
    }

    // pub fn get_by_id(id: EntityId) -> Option<VehicleContainer> {
    //     let manager = ENTITY_MANAGER_INSTANCE.get().unwrap().try_lock().unwrap();
    //     let result = manager.get_by_id(id);

    //     dbg!(result);

    //     match result {
    //         Some(_wrapper @ EntityWrapper::Vehicle(vehicle)) => Some(Rc::clone(&vehicle)),
    //         None | Some(_) => None,
    //     }
    // }

    pub fn set_secondary_color(&self, color: u8) -> Result<(), String> {
        unsafe { sdk::set_vehicle_primary_color(self.ptr.to_vehicle()?, color) };
        Ok(())
    }

    pub fn get_secondary_color(&self) -> Result<u8, String> {
        Ok(unsafe { sdk::get_vehicle_primary_color(self.ptr.to_vehicle()?) })
    }

    pub fn destroy(&mut self) -> Result<(), String> {
        // TODO:
        // ENTITY_MANAGER_INSTANCE
        //     .get()
        //     .unwrap()
        //     .try_lock()
        //     .unwrap()
        //     .on_destroy(self.ptr().to_entity().unwrap());

        self.destroy_base_object()
    }
}

impl BaseObject for Vehicle {
    fn as_any(&mut self) -> &mut dyn std::any::Any {
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

pub type VehicleContainer = Rc<RefCell<Vehicle>>;

pub fn create_vehicle(
    mut base_objects: RefMut<BaseObjectManager>,
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

    base_objects.on_create(base_object_raw_ptr, vehicle.clone());

    // TODO:
    // ENTITY_MANAGER_INSTANCE
    //     .get()
    //     .unwrap()
    //     .try_lock()
    //     .unwrap()
    //     .on_create(
    //         vehicle.borrow().id().unwrap(),
    //         EntityWrapper::Vehicle(Rc::clone),
    //     );

    Some(vehicle)
}

pub fn create_vehicle_container(raw_ptr: RawBaseObjectPointer) -> VehicleContainer {
    Rc::new(RefCell::new(Vehicle {
        ptr: BaseObjectPointer::new(raw_ptr),
        base_type: altv_sdk::BaseObjectType::VEHICLE,
    }))
}
