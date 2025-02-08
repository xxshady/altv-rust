// auto-generated from build.rs

#[repr(C)]
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub enum VehicleModelType {
    Invalid,
    Ped,
    Automobile,
    Plane,
    Trailer,
    QuadBike,
    SubmarineCar,
    AmphibiousAutomobile,
    AmphibiousQuadBike,
    Heli,
    Blimp,
    Autogyro,
    Bike,
    Bmx,
    Boat,
    Train,
    Submarine,
    Object,
}

impl TryFrom<u8> for VehicleModelType {
type Error = ();
fn try_from(v: u8) -> Result<Self, Self::Error> {
Ok(match v {
    v if v == Self::Invalid as u8 => Self::Invalid,
    v if v == Self::Ped as u8 => Self::Ped,
    v if v == Self::Automobile as u8 => Self::Automobile,
    v if v == Self::Plane as u8 => Self::Plane,
    v if v == Self::Trailer as u8 => Self::Trailer,
    v if v == Self::QuadBike as u8 => Self::QuadBike,
    v if v == Self::SubmarineCar as u8 => Self::SubmarineCar,
    v if v == Self::AmphibiousAutomobile as u8 => Self::AmphibiousAutomobile,
    v if v == Self::AmphibiousQuadBike as u8 => Self::AmphibiousQuadBike,
    v if v == Self::Heli as u8 => Self::Heli,
    v if v == Self::Blimp as u8 => Self::Blimp,
    v if v == Self::Autogyro as u8 => Self::Autogyro,
    v if v == Self::Bike as u8 => Self::Bike,
    v if v == Self::Bmx as u8 => Self::Bmx,
    v if v == Self::Boat as u8 => Self::Boat,
    v if v == Self::Train as u8 => Self::Train,
    v if v == Self::Submarine as u8 => Self::Submarine,
    v if v == Self::Object as u8 => Self::Object,_ => return Err(()),
})
}
}
