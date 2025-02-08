// auto-generated from build.rs

#[repr(C)]
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub enum ConfigValueType {
    None,
    String,
    Bool,
    Number,
    List,
    Dict,
}

impl TryFrom<u8> for ConfigValueType {
type Error = ();
fn try_from(v: u8) -> Result<Self, Self::Error> {
Ok(match v {
    v if v == Self::None as u8 => Self::None,
    v if v == Self::String as u8 => Self::String,
    v if v == Self::Bool as u8 => Self::Bool,
    v if v == Self::Number as u8 => Self::Number,
    v if v == Self::List as u8 => Self::List,
    v if v == Self::Dict as u8 => Self::Dict,_ => return Err(()),
})
}
}
