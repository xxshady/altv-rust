use crate::{helpers::assert_its_called_once, LocalVehicleManager, VehicleManager};

pub struct Api {
    pub local_vehicles: LocalVehicleManager,
    pub vehicles: VehicleManager,
}

impl Api {
    /// Can only be created once
    pub fn new() -> Self {
        assert_its_called_once!();

        Self {
            local_vehicles: LocalVehicleManager::default(),
            vehicles: VehicleManager::default(),
        }
    }
}
