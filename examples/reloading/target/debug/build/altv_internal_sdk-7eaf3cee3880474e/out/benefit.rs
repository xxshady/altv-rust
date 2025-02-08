// auto-generated from build.rs

#[repr(C)]
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub enum Benefit {
    None,
    CloudAuth,
}

impl TryFrom<u8> for Benefit {
type Error = ();
fn try_from(v: u8) -> Result<Self, Self::Error> {
Ok(match v {
    v if v == Self::None as u8 => Self::None,
    v if v == Self::CloudAuth as u8 => Self::CloudAuth,_ => return Err(()),
})
}
}
