// auto-generated from build.rs

#[repr(C)]
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub enum MValueType {
    None,
    Nil,
    Bool,
    Int,
    Uint,
    Double,
    String,
    List,
    Dict,
    BaseObject,
    Function,
    Vector3,
    Rgba,
    ByteArray,
    Vector2,
}

impl TryFrom<u8> for MValueType {
type Error = ();
fn try_from(v: u8) -> Result<Self, Self::Error> {
Ok(match v {
    v if v == Self::None as u8 => Self::None,
    v if v == Self::Nil as u8 => Self::Nil,
    v if v == Self::Bool as u8 => Self::Bool,
    v if v == Self::Int as u8 => Self::Int,
    v if v == Self::Uint as u8 => Self::Uint,
    v if v == Self::Double as u8 => Self::Double,
    v if v == Self::String as u8 => Self::String,
    v if v == Self::List as u8 => Self::List,
    v if v == Self::Dict as u8 => Self::Dict,
    v if v == Self::BaseObject as u8 => Self::BaseObject,
    v if v == Self::Function as u8 => Self::Function,
    v if v == Self::Vector3 as u8 => Self::Vector3,
    v if v == Self::Rgba as u8 => Self::Rgba,
    v if v == Self::ByteArray as u8 => Self::ByteArray,
    v if v == Self::Vector2 as u8 => Self::Vector2,_ => return Err(()),
})
}
}
