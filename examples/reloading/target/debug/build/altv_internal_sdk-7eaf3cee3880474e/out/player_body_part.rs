// auto-generated from build.rs

#[repr(C)]
            #[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
            pub enum PlayerBodyPart {
    Pelvis,
    LeftHip,
    LeftLeg,
    LeftFoot,
    RightHip,
    RightLeg,
    RightFoot,
    LowerTorso,
    UpperTorso,
    Chest,
    UnderNeck,
    LeftShoulder,
    LeftUpperArm,
    LeftElbrow,
    LeftWrist,
    RightShoulder,
    RightUpperArm,
    RightElbrow,
    RightWrist,
    Neck,
    Head,
    Unknown = -1,
}

impl TryFrom<i8> for PlayerBodyPart {
type Error = ();
fn try_from(v: i8) -> Result<Self, Self::Error> {
Ok(match v {
    v if v == Self::Pelvis as i8 => Self::Pelvis,
    v if v == Self::LeftHip as i8 => Self::LeftHip,
    v if v == Self::LeftLeg as i8 => Self::LeftLeg,
    v if v == Self::LeftFoot as i8 => Self::LeftFoot,
    v if v == Self::RightHip as i8 => Self::RightHip,
    v if v == Self::RightLeg as i8 => Self::RightLeg,
    v if v == Self::RightFoot as i8 => Self::RightFoot,
    v if v == Self::LowerTorso as i8 => Self::LowerTorso,
    v if v == Self::UpperTorso as i8 => Self::UpperTorso,
    v if v == Self::Chest as i8 => Self::Chest,
    v if v == Self::UnderNeck as i8 => Self::UnderNeck,
    v if v == Self::LeftShoulder as i8 => Self::LeftShoulder,
    v if v == Self::LeftUpperArm as i8 => Self::LeftUpperArm,
    v if v == Self::LeftElbrow as i8 => Self::LeftElbrow,
    v if v == Self::LeftWrist as i8 => Self::LeftWrist,
    v if v == Self::RightShoulder as i8 => Self::RightShoulder,
    v if v == Self::RightUpperArm as i8 => Self::RightUpperArm,
    v if v == Self::RightElbrow as i8 => Self::RightElbrow,
    v if v == Self::RightWrist as i8 => Self::RightWrist,
    v if v == Self::Neck as i8 => Self::Neck,
    v if v == Self::Head as i8 => Self::Head,
    v if v == Self::Unknown as i8 => Self::Unknown,_ => return Err(()),
})
}
}
