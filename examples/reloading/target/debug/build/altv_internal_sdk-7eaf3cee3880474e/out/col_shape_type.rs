// auto-generated from build.rs

#[repr(C)]
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub enum ColShapeType {
    Sphere,
    Cylinder,
    Circle,
    Cuboid,
    Rect,
    CheckpointCylinder,
    Polygon,
}

impl TryFrom<u8> for ColShapeType {
type Error = ();
fn try_from(v: u8) -> Result<Self, Self::Error> {
Ok(match v {
    v if v == Self::Sphere as u8 => Self::Sphere,
    v if v == Self::Cylinder as u8 => Self::Cylinder,
    v if v == Self::Circle as u8 => Self::Circle,
    v if v == Self::Cuboid as u8 => Self::Cuboid,
    v if v == Self::Rect as u8 => Self::Rect,
    v if v == Self::CheckpointCylinder as u8 => Self::CheckpointCylinder,
    v if v == Self::Polygon as u8 => Self::Polygon,_ => return Err(()),
})
}
}
