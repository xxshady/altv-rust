// auto-generated from build.rs

#[repr(C)]
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub enum BlipType {
    Vehicle = 1,
    Ped = 2,
    Object = 3,
    Destination = 4,
    Cont = 5,
    PickupUnk = 6,
    Radius = 7,
    Pickup = 8,
    Cop = 9,
    Area = 11,
    Gallery = 12,
    PickupObject = 13,
}

impl TryFrom<u8> for BlipType {
type Error = ();
fn try_from(v: u8) -> Result<Self, Self::Error> {
Ok(match v {
    v if v == Self::Vehicle as u8 => Self::Vehicle,
    v if v == Self::Ped as u8 => Self::Ped,
    v if v == Self::Object as u8 => Self::Object,
    v if v == Self::Destination as u8 => Self::Destination,
    v if v == Self::Cont as u8 => Self::Cont,
    v if v == Self::PickupUnk as u8 => Self::PickupUnk,
    v if v == Self::Radius as u8 => Self::Radius,
    v if v == Self::Pickup as u8 => Self::Pickup,
    v if v == Self::Cop as u8 => Self::Cop,
    v if v == Self::Area as u8 => Self::Area,
    v if v == Self::Gallery as u8 => Self::Gallery,
    v if v == Self::PickupObject as u8 => Self::PickupObject,_ => return Err(()),
})
}
}
