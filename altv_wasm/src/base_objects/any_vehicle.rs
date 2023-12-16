use super::objects::{vehicle::Vehicle, local_vehicle::LocalVehicle};

#[derive(Debug)]
pub enum AnyVehicle {
    Vehicle(Vehicle),
    LocalVehicle(LocalVehicle),
}
