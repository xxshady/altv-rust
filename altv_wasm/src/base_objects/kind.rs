#[derive(Debug)]
pub(crate) enum BaseObjectKind {
    /// A player or server vehicle as an example
    Other,

    /// Local vehicle created by other resource
    LocalVehicleUnknown,
    LocalVehicleStatic,
    LocalVehicleStreamed,
}
