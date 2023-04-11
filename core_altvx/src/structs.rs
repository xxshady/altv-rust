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
