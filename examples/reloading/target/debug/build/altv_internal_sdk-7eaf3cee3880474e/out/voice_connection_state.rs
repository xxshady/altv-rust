// auto-generated from build.rs

#[repr(C)]
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub enum VoiceConnectionState {
    Disconnected,
    Connecting,
    Connected,
}

impl TryFrom<u8> for VoiceConnectionState {
type Error = ();
fn try_from(v: u8) -> Result<Self, Self::Error> {
Ok(match v {
    v if v == Self::Disconnected as u8 => Self::Disconnected,
    v if v == Self::Connecting as u8 => Self::Connecting,
    v if v == Self::Connected as u8 => Self::Connected,_ => return Err(()),
})
}
}
