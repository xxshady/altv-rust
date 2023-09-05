use crate::__imports;
use super::base::private::Ptr;

pub trait SharedVehicle: Ptr {
    fn fuel_level(&self) -> f32 {
        __imports::vehicle_get_fuel_level(self.ptr())
    }

    fn set_fuel_level(&self, value: f32) {
        __imports::vehicle_set_fuel_level(self.ptr(), value);
    }
}
