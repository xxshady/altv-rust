use crate::vector::Vector3;

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
