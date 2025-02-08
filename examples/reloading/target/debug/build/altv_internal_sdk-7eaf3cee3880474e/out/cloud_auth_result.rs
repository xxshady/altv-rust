// auto-generated from build.rs

#[repr(C)]
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub enum CloudAuthResult {
    Success,
    NoBenefit,
    VerifyFailed,
}

impl TryFrom<u8> for CloudAuthResult {
type Error = ();
fn try_from(v: u8) -> Result<Self, Self::Error> {
Ok(match v {
    v if v == Self::Success as u8 => Self::Success,
    v if v == Self::NoBenefit as u8 => Self::NoBenefit,
    v if v == Self::VerifyFailed as u8 => Self::VerifyFailed,_ => return Err(()),
})
}
}
