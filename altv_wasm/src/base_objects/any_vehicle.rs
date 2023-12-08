use super::objects::{vehicle::Vehicle, local_vehicle::LocalVehicle};

#[derive(Debug)]
pub enum AnyVehicle {
    Server(Vehicle),
    Local(LocalVehicle),
}
