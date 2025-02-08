// auto-generated from build.rs

#[repr(C)]
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub enum PlayerConnectDeniedReason {
    WrongVersion,
    WrongBranch,
    DebugNotAllowed,
    WrongPassword,
    WrongCdnUrl,
}

impl TryFrom<u8> for PlayerConnectDeniedReason {
type Error = ();
fn try_from(v: u8) -> Result<Self, Self::Error> {
Ok(match v {
    v if v == Self::WrongVersion as u8 => Self::WrongVersion,
    v if v == Self::WrongBranch as u8 => Self::WrongBranch,
    v if v == Self::DebugNotAllowed as u8 => Self::DebugNotAllowed,
    v if v == Self::WrongPassword as u8 => Self::WrongPassword,
    v if v == Self::WrongCdnUrl as u8 => Self::WrongCdnUrl,_ => return Err(()),
})
}
}
