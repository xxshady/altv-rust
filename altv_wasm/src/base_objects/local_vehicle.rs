use altv_wasm_shared::BaseObjectPtr;

use crate::{__imports, state::State};
use super::{
    base::private::Ptr,
    objects::local_vehicle::LocalVehicle,
    shared_vehicle::SharedVehicle,
    world_object::{WorldObject, ClientWorldObject},
};

#[derive(Debug, Default)]
pub struct LocalVehicleManager {
    objects: Vec<LocalVehicle>,
}

impl LocalVehicleManager {
    pub fn create(
        &mut self,
        model: u32,
        dimension: i32,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        rot_x: f32,
        rot_y: f32,
        rot_z: f32,
        use_streaming: bool,
        streaming_distance: u32,
    ) -> LocalVehicle {
        let ptr = __imports::create_local_vehicle(
            model,
            dimension,
            pos_x,
            pos_y,
            pos_z,
            rot_x,
            rot_y,
            rot_z,
            use_streaming,
            streaming_distance,
        );

        State::with_base_objects_mut(|mut base_objects, _| {
            base_objects.on_create(ptr, altv_wasm_shared::BaseObjectType::LocalVehicle);
        });

        LocalVehicle::new(ptr)
    }

    pub fn all(&mut self) -> &[LocalVehicle] {
        State::with_base_objects_ref(|base_objects, _| {
            self.objects = base_objects
                .local_vehicle_iter()
                .map(|ptr| LocalVehicle::new(*ptr))
                .collect();
        });

        &self.objects
    }

    pub fn destroy(&mut self, object: LocalVehicle) {
        State::with_base_objects_mut(|mut base_objects, _| {
            base_objects.on_destroy(object.ptr(), altv_wasm_shared::BaseObjectType::LocalVehicle);
        });

        __imports::destroy_base_object(object.ptr());
    }

    pub fn get_by_token(&mut self, token: &LocalVehicleToken) -> Option<&LocalVehicle> {
        self.all().iter().find(|v| v.ptr() == token.0)
    }
}

#[derive(Debug)]
pub struct LocalVehicleToken(pub(crate) BaseObjectPtr);

impl LocalVehicle {
    pub fn destroy(self, manager: &mut LocalVehicleManager) {
        manager.destroy(self);
    }
}

impl SharedVehicle for LocalVehicle {}
impl WorldObject for LocalVehicle {}
impl ClientWorldObject for LocalVehicle {}
