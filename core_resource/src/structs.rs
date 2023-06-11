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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

impl std::ops::BitOr for AnimationFlags {
    type Output = i32;
    fn bitor(self, rhs: Self) -> Self::Output {
        (self as i32) | (rhs as i32)
    }
}

impl std::ops::BitAnd for AnimationFlags {
    type Output = i32;
    fn bitand(self, rhs: Self) -> Self::Output {
        (self as i32) & (rhs as i32)
    }
}

impl std::ops::BitXor for AnimationFlags {
    type Output = i32;
    fn bitxor(self, rhs: Self) -> Self::Output {
        (self as i32) ^ (rhs as i32)
    }
}

#[derive(Debug)]
pub struct VehicleNeon {
    pub left: bool,
    pub right: bool,
    pub front: bool,
    pub back: bool,
}

// https://gist.github.com/root-cause/faf41f59f7a6d818b7db0b839bd147c1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AmmoType {
    Pistol = 0x743d4f54,
    Smg = 0x6c7d23b8,
    Rifle = 0xd05319f,
    Mg = 0x6aa1343f,
    Shotgun = 0x90083d3b,
    Stungun = 0xb02eade0,
    Sniper = 0x4c98087b,
    SniperRemote = 0xfeda7d30,
    GrenadeLauncher = 0x3bcca5ee,
    GrenadeLauncherSmoke = 0x313fd340,
    Rpg = 0x67dd81f2,
    Stinger = 0x914c813a,
    Minigun = 0x9fc5c882,
    Grenade = 0x3bd313b1,
    StickyBomb = 0x5424b617,
    SmokeGrenade = 0xe60e08a6,
    Bzgas = 0x9b747ea4,
    Molotov = 0x5633f9d5,
    Fireextinguisher = 0x5106b43c,
    Petrolcan = 0xca6318a1,
    Ball = 0xff956666,
    Flare = 0x6bccf76f,
    Tank = 0xa81b4220,
    SpaceRocket = 0x1f75106c,
    PlaneRocket = 0x47735976,
    PlayerLaser = 0xf624d80a,
    EnemyLaser = 0xae2ea48c,
    BirdCrap = 0x4298c094,
    Pipebomb = 0x155663f8,
    HomingLauncher = 0x99150e2d,
    ProxMine = 0xaf2208a7,
    Snowball = 0x8218416d,
    RifleArmorpiercing = 0x1941d244,
    RifleFmj = 0x5e962ddc,
    RifleIncendiary = 0x92f129cd,
    RifleTracer = 0xb0198d5f,
    SniperArmorpiercing = 0xa6bcbda9,
    SniperFmj = 0xf5f1c616,
    SniperIncendiary = 0x2f7ca4a6,
    SniperTracer = 0x469293cd,
    ShotgunArmorpiercing = 0x72a3a760,
    ShotgunExplosive = 0xed906955,
    ShotgunHollowpoint = 0x7c867272,
    ShotgunIncendiary = 0xdbacd794,
    PistolFmj = 0xbc7af403,
    PistolHollowpoint = 0xce23b916,
    PistolIncendiary = 0xab8ea0f9,
    PistolTracer = 0xb8dcee2b,
    MgArmorpiercing = 0x2ec80a10,
    MgFmj = 0xdfd80b5,
    MgIncendiary = 0x57237470,
    MgTracer = 0x4919b4eb,
    SniperExplosive = 0xadd16cb9,
    SmgFmj = 0x2d31add9,
    SmgHollowpoint = 0x27f43e92,
    SmgIncendiary = 0xec2875e7,
    SmgTracer = 0x5d9106d1,
    Flaregun = 0x45f0e965,
    Firework = 0xaf23ee0f,
    Railgun = 0x794446fd,
}
