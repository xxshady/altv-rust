use serde::{Serialize, Deserialize};

use crate::BaseObjectPtr;

#[derive(Serialize, Deserialize)]
pub enum RawEvent {
    EnteredVehicle { vehicle: BaseObjectPtr, seat: u8 },
}
