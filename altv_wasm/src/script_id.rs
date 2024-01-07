pub trait AsEntityScriptId {
    fn as_entity_script_id(&self) -> u32;
}

impl<T> AsEntityScriptId for T
where
    T: AsVehicleScriptId,
{
    fn as_entity_script_id(&self) -> u32 {
        self.as_vehicle_script_id()
    }
}

pub trait AsVehicleScriptId {
    fn as_vehicle_script_id(&self) -> u32;
}
