use crate::{helpers::Hash, vector::Vector3};

#[derive(Debug)]
pub struct FireInfo {
    pub pos: Vector3,
    pub weapon_hash: Hash,
}
