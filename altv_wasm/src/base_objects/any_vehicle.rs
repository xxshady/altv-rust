use crate::LocalVehicleManager;

use super::objects::{vehicle::Vehicle, local_vehicle::LocalVehicleLocked};

#[derive(Debug)]
pub enum AnyVehicle {
    Server(Vehicle),
    Local(fn(&LocalVehicleManager) -> LocalVehicleLocked<LocalVehicleManager>),
}
