use altv_wasm_shared::BaseObjectPtr;

use crate::{State, __imports};
use super::{
    objects::vehicle::Vehicle, remote::RemoteBaseObject, shared_vehicle::SharedVehicle,
    world_object::WorldObject, script_id::VehicleScriptId,
};

impl Vehicle {
    /// Read all **spawned** vehicles.<br>
    /// ### Why does it take a closure?
    /// To eliminate holding `&[Vehicle]` for longer than a tick at **compile time**.
    /// Consider this example if we would have `altv::Vehicle::all() -> &[Vehicle]` instead:
    /// ```
    /// async {
    ///     let [veh] = altv::Vehicle::all_spawned() else {
    ///         unreachable!()
    ///     };
    ///     altv::wait(Duration::from_secs(10)).await;
    ///     // vehicle may already be despawned by serverside here
    ///     veh.id();
    /// };
    /// ```
    ///
    /// # Example
    /// ```
    /// let count = altv::Vehicle::read_all_spawned(|vehicles| vehicles.len());
    /// altv::log!("vehicles count: {count}");
    /// ```
    pub fn read_all_spawned<R>(reader: impl FnOnce(&[Vehicle]) -> R) -> R {
        let vehicles = State::with_base_objects_ref(|objects, _| {
            objects
                .vehicle
                .iter()
                .filter_map(|ptr| Self::new(*ptr))
                .collect::<Vec<_>>()
        });

        reader(&vehicles)
    }

    /// Returns `None` if vehicle is not spawned
    pub fn read_by_id<R>(id: u32, reader: impl FnOnce(&Vehicle) -> R) -> Option<R> {
        Self::read_all_spawned(|all| {
            let object = all.iter().find(|v| v.id() == id);
            let Some(object) = object else {
                return None;
            };
            Some(reader(object))
        })
    }

    /// Returns `None` if vehicle is not spawned
    pub fn read_by_remote_id<R>(id: u32, reader: impl FnOnce(&Vehicle) -> R) -> Option<R> {
        Self::read_all_spawned(|all| {
            let object = all.iter().find(|v| v.remote_id() == id);
            let Some(object) = object else {
                return None;
            };
            Some(reader(object))
        })
    }

    pub(crate) fn new(ptr: BaseObjectPtr) -> Option<Self> {
        let script_id = __imports::entity_get_script_id(ptr);
        if script_id != 0 {
            Some(Vehicle::internal_new_borrowed(
                ptr,
                VehicleData {
                    script_id: VehicleScriptId(script_id),
                },
            ))
        } else {
            None
        }
    }

    /// Vehicle instances are guaranteed to be spawned
    pub fn script_id(&self) -> &VehicleScriptId {
        &self.data.script_id
    }
}

impl RemoteBaseObject for Vehicle {}
impl SharedVehicle for Vehicle {}
impl WorldObject for Vehicle {}

#[derive(Debug)]
pub struct VehicleData {
    pub(crate) script_id: VehicleScriptId,
}
