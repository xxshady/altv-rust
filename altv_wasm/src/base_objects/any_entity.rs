use super::objects::{vehicle::Vehicle, local_vehicle::LocalVehicle};

#[derive(Debug)]
pub enum AnyEntity {
    Vehicle(Vehicle),
    LocalVehicle(LocalVehicle),
}
