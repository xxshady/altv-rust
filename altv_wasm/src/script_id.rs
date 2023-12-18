#[derive(Debug)]
pub struct VehicleScriptId(pub u32);

pub trait AsEntityScriptId {
    fn as_entity_script_id(self) -> u32;
}

impl AsEntityScriptId for &VehicleScriptId {
    fn as_entity_script_id(self) -> u32 {
        self.0
    }
}
