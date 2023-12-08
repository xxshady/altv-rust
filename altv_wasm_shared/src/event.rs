use serde::{Serialize, Deserialize};

use crate::{BaseObjectPtr, BaseObjectType};

#[derive(Serialize, Deserialize)]
pub enum RawEvent {
    EnteredVehicle {
        vehicle: (BaseObjectPtr, BaseObjectType),
        seat: u8,
    },
}
