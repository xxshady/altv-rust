use serde::{Serialize, Deserialize};

use crate::{BaseObjectPtr, BaseObjectType};

pub type BaseObjectWithType = (BaseObjectPtr, BaseObjectType);

#[derive(Serialize, Deserialize)]
pub enum RawEvent {
    EnteredVehicle {
        vehicle: BaseObjectWithType,
        seat: u8,
    },
    GameEntityCreate {
        entity: BaseObjectWithType,
    },
    GameEntityDestroy {
        entity: BaseObjectWithType,
    },
}
