use std::time::Duration;

use altv_wasm_shared::BaseObjectPtr;
use anyhow::{bail, anyhow};

use crate::{__imports, state::State, wait_for, SomeResult, event};
use super::{
    objects::{local_vehicle::LocalVehicle, BaseObjectManager},
    spawned_vehicle::SpawnedVehicle,
    world_object::{WorldObject, ClientWorldObject},
    kind::BaseObjectKind,
    base::private::Ptr,
    any_entity::AnyEntity,
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

        let instance = State::with_base_objects_mut(|mut base_objects, _| {
            base_objects.on_create(
                ptr,
                altv_wasm_shared::BaseObjectType::LocalVehicle,
                BaseObjectKind::LocalVehicleStatic,
            );
            LocalVehicle::internal_new_owned(ptr, &mut base_objects.all, ())
        });

        // TODO: use game entity create event
        let spawn_res = wait_for(
            || __imports::entity_get_script_id(ptr) != 0,
            Duration::from_secs(15),
        )
        .await;

        match spawn_res {
            Ok(_) => Ok(LocalVehicleStatic::new(instance, ptr)),
            Err(_) => {
                // TODO: destroy base object?
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
        on_spawn: impl FnMut(&SpawnedLocalVehicleStreamed) + 'static,
        on_despawn: impl FnMut() + 'static,
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
        Ok(LocalVehicleStreamed::new(
            instance, ptr, on_spawn, on_despawn,
        ))
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

#[must_use = "Instance is immediately destroyed if ignored"]
#[derive(Debug)]
pub struct LocalVehicleStatic {
    instance: LocalVehicle,
}

impl LocalVehicleStatic {
    pub(crate) fn new(instance: LocalVehicle, ptr: BaseObjectPtr) -> Self {
        Self { instance }
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

impl Ptr for &LocalVehicleStatic {
    fn ptr(&self) -> BaseObjectPtr {
        self.instance.ptr()
    }
}

impl SpawnedVehicle for &LocalVehicleStatic {}

#[must_use = "Instance is immediately destroyed if ignored"]
#[derive(Debug)]
pub struct LocalVehicleStreamed {
    instance: LocalVehicle,

    // option because of Drop
    event_controllers: Option<[event::EventController; 2]>,
}

impl LocalVehicleStreamed {
    pub(crate) fn new(
        instance: LocalVehicle,
        ptr: BaseObjectPtr,
        mut on_spawn: impl FnMut(&SpawnedLocalVehicleStreamed) + 'static,
        mut on_despawn: impl FnMut() + 'static,
    ) -> Self {
        // TODO: static event handlers for these
        Self {
            instance,
            event_controllers: Some([
                event::add_handler(event::EventHandler::GameEntityCreate(Box::new(
                    move |ctx| {
                        logger::debug!("GameEntityCreate");
                        let AnyEntity::LocalVehicle(ref veh) = ctx.entity else {
                            return;
                        };
                        if veh.ptr() != ptr {
                            return;
                        }

                        let script_id = __imports::entity_get_script_id(ptr);
                        assert!(script_id != 0);
                        on_spawn(&SpawnedLocalVehicleStreamed(ptr));
                    },
                ))),
                event::add_handler(event::EventHandler::GameEntityDestroy(Box::new(
                    move |ctx| {
                        logger::debug!("GameEntityDestroy");
                        let AnyEntity::LocalVehicle(ref veh) = ctx.entity else {
                            return;
                        };
                        if veh.ptr() != ptr {
                            return;
                        }
                        on_despawn();
                    },
                ))),
            ]),
        }
    }
}

impl Drop for LocalVehicleStreamed {
    fn drop(&mut self) {
        for c in self.event_controllers.take().unwrap() {
            c.destroy();
        }
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

#[derive(Debug)]
pub struct SpawnedLocalVehicleStreamed(BaseObjectPtr);

impl Ptr for &SpawnedLocalVehicleStreamed {
    fn ptr(&self) -> BaseObjectPtr {
        self.0
    }
}

impl SpawnedVehicle for &SpawnedLocalVehicleStreamed {}
