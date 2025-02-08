// auto-generated from build.rs

#[repr(C)]
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub enum AmmoSpecialType {
    None,
    ArmorPiercing,
    Explosive,
    FullMetalJacket,
    HollowPoint,
    Incendiary,
    Tracer,
}

impl TryFrom<u32> for AmmoSpecialType {
type Error = ();
fn try_from(v: u32) -> Result<Self, Self::Error> {
Ok(match v {
    v if v == Self::None as u32 => Self::None,
    v if v == Self::ArmorPiercing as u32 => Self::ArmorPiercing,
    v if v == Self::Explosive as u32 => Self::Explosive,
    v if v == Self::FullMetalJacket as u32 => Self::FullMetalJacket,
    v if v == Self::HollowPoint as u32 => Self::HollowPoint,
    v if v == Self::Incendiary as u32 => Self::Incendiary,
    v if v == Self::Tracer as u32 => Self::Tracer,_ => return Err(()),
})
}
}
