use crate::{__imports, state::State};
use super::{
    objects::{local_vehicle::LocalVehicle, BaseObjectManager},
    shared_vehicle::SharedVehicle,
    world_object::{WorldObject, ClientWorldObject},
};

impl LocalVehicle {
    pub fn new(
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
            LocalVehicle::internal_new_owned(ptr, &mut base_objects.all)
        })
    }

    pub fn read_all<R>(reader: impl FnOnce(&[LocalVehicle]) -> R) -> R {
        let vehicles = State::with(|state| {
            // wtf did not know it was possible
            let BaseObjectManager {
                all,
                local_vehicle,
                vehicle: _,
            } = &mut *state.base_objects.borrow_mut();

            let ptrs = local_vehicle.iter();
            ptrs.map(|ptr| LocalVehicle::internal_new_owned(*ptr, all))
                .collect::<Vec<_>>()
        });

        reader(&vehicles)
    }
}

impl SharedVehicle for LocalVehicle {}
impl WorldObject for LocalVehicle {}
impl ClientWorldObject for LocalVehicle {}
