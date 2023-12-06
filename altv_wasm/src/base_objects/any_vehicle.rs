use super::{objects::vehicle::Vehicle, local_vehicle::LocalVehicleToken};

#[derive(Debug)]
pub enum AnyVehicle {
    Server(Vehicle),
    Local(LocalVehicleToken),
}
