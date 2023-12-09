use crate::State;
use super::{
    objects::vehicle::Vehicle, remote::RemoteBaseObject, shared_vehicle::SharedVehicle,
    world_object::WorldObject,
};

impl Vehicle {
    /// Read all created objects.<br>
    /// ### Why does it take a closure?
    /// To eliminate holding `&[Vehicle]` for longer than a tick at **compile time**.
    /// Consider this example if we would have `altv::Vehicle::all() -> &[Vehicle]` instead:
    /// ```
    /// async {
    ///     let [veh] = altv::Vehicle::all() else {
    ///         unreachable!()
    ///     };
    ///     altv::wait(Duration::from_secs(10)).await;
    ///     // vehicle may already be destroyed by serverside here
    ///     veh.id();
    /// };
    /// ```
    ///
    /// # Example
    /// ```
    /// let count = altv::Vehicle::read_all(|vehicles| vehicles.len());
    /// altv::log!("vehicles count: {count}");
    /// ```
    pub fn read_all<R>(reader: impl FnOnce(&[Vehicle]) -> R) -> R {
        let vehicles = State::with_base_objects_ref(|objects, _| {
            objects
                .vehicle
                .iter()
                .map(|ptr| Vehicle::internal_new_borrowed(*ptr))
                .collect::<Vec<_>>()
        });

        reader(&vehicles)
    }

    /// Returns `None` if object was not found
    pub fn read_by_id<R>(id: u32, reader: impl FnOnce(&Vehicle) -> R) -> Option<R> {
        Self::read_all(|all| {
            let object = all.iter().find(|v| v.id() == id);
            let Some(object) = object else {
                return None;
            };
            Some(reader(object))
        })
    }

    /// Returns `None` if object was not found
    pub fn read_by_remote_id<R>(id: u32, reader: impl FnOnce(&Vehicle) -> R) -> Option<R> {
        Self::read_all(|all| {
            let object = all.iter().find(|v| v.remote_id() == id);
            let Some(object) = object else {
                return None;
            };
            Some(reader(object))
        })
    }
}

impl RemoteBaseObject for Vehicle {}
impl SharedVehicle for Vehicle {}
impl WorldObject for Vehicle {}
