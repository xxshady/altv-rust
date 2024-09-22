use super::{instance::BaseObject, scoped_instance::ScopedBaseObject};

#[derive(Clone)]
pub struct BlipType;

pub type Blip = BaseObject<BlipType>;
pub type ScopedBlip<'scope> = ScopedBaseObject<'scope, BlipType>;
