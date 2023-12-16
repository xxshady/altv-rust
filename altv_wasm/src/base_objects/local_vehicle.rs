use std::time::Duration;

use altv_wasm_shared::BaseObjectPtr;
use anyhow::{bail, anyhow};

use crate::{__imports, state::State, wait_for, SomeResult};
use super::{
    objects::{local_vehicle::LocalVehicle, BaseObjectManager},
    shared_vehicle::SharedVehicle,
    world_object::{WorldObject, ClientWorldObject},
    script_id::VehicleScriptId,
    kind::BaseObjectKind,
    base::private::Ptr,
};

impl LocalVehicle {
    pub async fn new_static(
        model: u32,
        dimension: i32,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        rot_x: f32,
        rot_y: f32,
        rot_z: f32,
    ) -> SomeResult<LocalVehicleStatic> {
        let ptr = Self::create(
            model, dimension, pos_x, pos_y, pos_z, rot_x, rot_y, rot_z, false, 0,
        )?;

        // TODO: use game entity create event
        let spawn_res = wait_for(
            || __imports::entity_get_script_id(ptr) != 0,
            Duration::from_secs(15),
        )
        .await;

        match spawn_res {
            Ok(_) => {
                let instance = State::with_base_objects_mut(|mut base_objects, _| {
                    base_objects.on_create(
                        ptr,
                        altv_wasm_shared::BaseObjectType::LocalVehicle,
                        BaseObjectKind::LocalVehicleStatic,
                    );
                    LocalVehicle::internal_new_owned(ptr, &mut base_objects.all, ())
                });
                Ok(LocalVehicleStatic::new(instance, ptr))
            }
            Err(_) => {
                bail!("Failed to wait for spawn of vehicle")
            }
        }
    }

    pub fn new_streamed(
        model: u32,
        dimension: i32,
        pos_x: f32,
        pos_y: f32,
        pos_z: f32,
        rot_x: f32,
        rot_y: f32,
        rot_z: f32,
        streaming_distance: u32,
        on_spawn: impl FnMut(&VehicleScriptId),
        on_despawn: impl FnMut(&VehicleScriptId),
    ) -> SomeResult<LocalVehicleStreamed> {
        let ptr = Self::create(
            model,
            dimension,
            pos_x,
            pos_y,
            pos_z,
            rot_x,
            rot_y,
            rot_z,
            true,
            streaming_distance,
        )?;

        let instance = State::with_base_objects_mut(|mut base_objects, _| {
            base_objects.on_create(
                ptr,
                altv_wasm_shared::BaseObjectType::LocalVehicle,
                BaseObjectKind::LocalVehicleStreamed,
            );
            LocalVehicle::internal_new_owned(ptr, &mut base_objects.all, ())
        });
        Ok(LocalVehicleStreamed::new(instance, on_spawn, on_despawn))
    }

    fn create(
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
    ) -> SomeResult<BaseObjectPtr> {
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

        if ptr == 0 {
            Err(anyhow!("Failed to create local vehicle"))
        } else {
            Ok(ptr)
        }
    }

    pub fn read_all<R>(reader: impl FnOnce(&[LocalVehicle]) -> R) -> R {
        let vehicles = State::with(|state| {
            // wtf did not know it was possible
            let BaseObjectManager {
                all,
                local_vehicle,
                vehicle: _,
            } = &mut *state.base_objects.borrow_mut();

            local_vehicle
                .iter()
                .map(|ptr| LocalVehicle::internal_new_owned(*ptr, all, ()))
                .collect::<Vec<_>>()
        });

        reader(&vehicles)
    }
}

impl SharedVehicle for LocalVehicle {}
impl WorldObject for LocalVehicle {}
impl ClientWorldObject for LocalVehicle {}

impl PartialEq for LocalVehicle {
    fn eq(&self, other: &Self) -> bool {
        self.ptr() == other.ptr()
    }
}

impl Eq for LocalVehicle {}

impl PartialEq<LocalVehicleStreamed> for LocalVehicle {
    fn eq(&self, other: &LocalVehicleStreamed) -> bool {
        self.ptr() == other.ptr()
    }
}

impl PartialEq<LocalVehicleStatic> for LocalVehicle {
    fn eq(&self, other: &LocalVehicleStatic) -> bool {
        self.ptr() == other.ptr()
    }
}

#[derive(Debug)]
pub struct LocalVehicleStatic {
    instance: LocalVehicle,
    script_id: VehicleScriptId,
}

impl LocalVehicleStatic {
    pub(crate) fn new(instance: LocalVehicle, ptr: BaseObjectPtr) -> Self {
        Self {
            instance,
            script_id: {
                let raw = __imports::entity_get_script_id(ptr);
                assert!(raw != 0);
                VehicleScriptId(raw)
            },
        }
    }

    pub fn script_id(&self) -> &VehicleScriptId {
        &self.script_id
    }
}

impl std::ops::Deref for LocalVehicleStatic {
    type Target = LocalVehicle;
    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl std::ops::DerefMut for LocalVehicleStatic {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instance
    }
}

#[derive(Debug)]
pub struct LocalVehicleStreamed {
    instance: LocalVehicle,
}

impl LocalVehicleStreamed {
    pub(crate) fn new(
        instance: LocalVehicle,
        on_spawn: impl FnMut(&VehicleScriptId),
        on_despawn: impl FnMut(&VehicleScriptId),
    ) -> Self {
        Self { instance }
    }
}

impl Drop for LocalVehicleStreamed {
    fn drop(&mut self) {
        // TODO: remove event listeners
    }
}

impl std::ops::Deref for LocalVehicleStreamed {
    type Target = LocalVehicle;
    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl std::ops::DerefMut for LocalVehicleStreamed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.instance
    }
}
