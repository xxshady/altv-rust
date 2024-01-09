use std::num::NonZeroU32;

use crate::{__imports, script_id::AsVehicleScriptId};
use super::base::private::Ptr;

pub trait SpawnedVehicle: Ptr {
    fn fuel_level(&self) -> f32 {
        __imports::vehicle_get_fuel_level(self.ptr())
    }

    fn set_fuel_level(&self, value: f32) {
        __imports::vehicle_set_fuel_level(self.ptr(), value);
    }

    fn seat_count(&self) -> u8 {
        __imports::vehicle_get_seat_count(self.ptr())
    }
}

impl<T> AsVehicleScriptId for &T
where
    T: SpawnedVehicle,
{
    fn as_vehicle_script_id(&self) -> u32 {
        // TODO: cache it
        NonZeroU32::new(__imports::entity_get_script_id(self.ptr()))
            .unwrap()
            .get()
    }
}
