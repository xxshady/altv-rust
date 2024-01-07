use altv_wasm_shared::BaseObjectPtr;

use crate::{State, __imports};
use super::{
    objects::vehicle::Vehicle, remote::RemoteBaseObject, spawned_vehicle::SpawnedVehicle,
    world_object::WorldObject,
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
        assert!(script_id != 0);

        if script_id != 0 {
            Some(Vehicle::internal_new_borrowed(ptr, VehicleData {}))
        } else {
            None
        }
    }
}

impl RemoteBaseObject for Vehicle {}
impl SpawnedVehicle for Vehicle {}
impl WorldObject for Vehicle {}

#[derive(Debug)]
pub struct VehicleData {}
