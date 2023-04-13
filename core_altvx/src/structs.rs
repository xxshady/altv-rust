use crate::{helpers::Hash, vector::Vector3};

#[derive(Debug, Default)]
pub struct AttachToEntityBoneIndex {
    pub other_bone_index: i16,
    pub my_bone_index: i16,
    pub pos: Vector3,
    pub rot: Vector3,
    pub collision: bool,
    pub no_fixed_rotation: bool,
}

#[derive(Debug, Default)]
pub struct AttachToEntityBoneName {
    pub other_bone_name: String,
    pub my_bone_name: String,
    pub pos: Vector3,
    pub rot: Vector3,
    pub collision: bool,
    pub no_fixed_rotation: bool,
}

#[derive(Debug, Default)]
pub struct PlayerDateTime {
    pub day: i32,
    pub month: i32,
    pub year: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
}

#[derive(Debug, Default)]
pub struct PlayerHeadBlendData {
    pub shape_first_id: u32,
    pub shape_second_id: u32,
    pub shape_third_id: u32,
    pub skin_first_id: u32,
    pub skin_second_id: u32,
    pub skin_third_id: u32,
    pub shape_mix: f32,
    pub skin_mix: f32,
    pub third_mix: f32,
}

#[derive(Debug)]
pub struct BoneInfo {
    pub id: u16,
    pub index: u16,
    pub name: String,
}

#[derive(Debug)]
pub struct PlayerCloth {
    pub drawable: u16,
    pub texture: u8,
    pub palette: u8,
}

#[derive(Debug)]
pub struct PlayerDlcCloth {
    pub drawable: u16,
    pub texture: u8,
    pub palette: u8,
    pub dlc: u32,
}

#[derive(Debug)]
pub struct PlayerProp {
    pub drawable: u16,
    pub texture: u8,
}

#[derive(Debug)]
pub struct PlayerDlcProp {
    pub drawable: u8,
    pub texture: u8,
    pub dlc: u32,
}

#[derive(Debug)]
pub struct PlayerHeadOverlay {
    pub index: u8,
    pub opacity: f32,
    pub color_type: u8,
    pub color_index: u8,
    pub second_color_index: u8,
}

#[derive(Debug)]
pub struct Weapon {
    pub hash: Hash,
    pub tint_index: u8,
    pub components: Vec<u32>,
}

#[derive(Debug)]
pub struct PlayAnimation {
    pub blend_in_speed: f32,
    pub blend_out_speed: f32,
    pub duration: i32,
    pub flags: i32,
    pub playback_rate: f32,
    pub lock_x: bool,
    pub lock_y: bool,
    pub lock_z: bool,
}

impl Default for PlayAnimation {
    fn default() -> Self {
        Self {
            blend_in_speed: 8.0,
            blend_out_speed: 8.0,
            duration: -1,
            flags: AnimationFlags::Looping as i32,
            playback_rate: 1.0,
            lock_x: false,
            lock_y: false,
            lock_z: false,
        }
    }
}

pub enum AnimationFlags {
    Default = 0,
    Looping = 1,
    HoldLastFrame = 2,
    RepositionWhenFinished = 4,
    NotInterruptable = 8,
    Upperbody = 16,
    Secondary = 32,
    ReorientWhenFinished = 64,
    AbortOnPedMovement = 128,
    Additive = 256,
    TurnOffCollision = 512,
    OverridePhysics = 1024,
    IgnoreGravity = 2048,
    ExtractInitialOffset = 4096,
    ExitAfterInterrupted = 8192,
    TagSyncIn = 16384,
    TagSyncOut = 32768,
    TagSyncContinuous = 65536,
    ForceStart = 131072,
    UseKinematicPhysics = 262144,
    UseMoverExtraction = 524288,
    HideWeapon = 1048576,
    EndsInDeadPose = 2097152,
    ActivateRagdollOnCollision = 4194304,
    DontExitOnDeath = 8388608,
    AbortOnWeaponDamage = 16777216,
    DisableForcedPhysicsUpdate = 33554432,
    ProcessAttachmentsOnStart = 67108864,
    ExpandPedCapsuleFromSkeleton = 134217728,
    UseAlternativeFpAnim = 268435456,
    BlendoutWrtLastFrame = 536870912,
    UseFullBlending = 1073741824,
}

#[derive(Debug)]
pub struct VehicleNeon {
    pub left: bool,
    pub right: bool,
    pub front: bool,
    pub back: bool,
}
