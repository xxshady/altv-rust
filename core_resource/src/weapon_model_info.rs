use crate::{
    helpers::{IntoHash, Hash},
    sdk,
};

#[derive(Debug)]
pub struct WeaponModelInfo {
    pub hash: Hash,
    pub name: String,
    pub model_name: String,
    pub model_hash: Hash,
    pub ammo_type_hash: Hash,
    pub ammo_type: String,
    pub ammo_model_name: String,
    pub ammo_model_hash: Hash,
    pub default_max_ammo_mp: i32,
    pub skill_above_50_max_ammo_mp: i32,
    pub max_skill_max_ammo_mp: i32,
    pub bonus_max_ammo_mp: i32,
}

impl WeaponModelInfo {
    pub fn get_by_hash(hash: impl IntoHash) -> Option<Self> {
        let ptr = unsafe { sdk::ICore::GetWeaponModelByHash(hash.into_hash()) };

        if !unsafe { sdk::is_weapon_model_info_valid(ptr) } {
            return None;
        }

        let (
            mut hash,
            mut model_hash,
            mut ammo_type_hash,
            mut ammo_model_hash,
            mut default_max_ammo_mp,
            mut skill_above_50_max_ammo_mp,
            mut max_skill_max_ammo_mp,
            mut bonus_max_ammo_mp,
        ) = Default::default();

        unsafe {
            sdk::read_weapon_model_info(
                ptr,
                &mut hash,
                &mut model_hash,
                &mut ammo_type_hash,
                &mut ammo_model_hash,
                &mut default_max_ammo_mp,
                &mut skill_above_50_max_ammo_mp,
                &mut max_skill_max_ammo_mp,
                &mut bonus_max_ammo_mp,
            )
        }

        Some(Self {
            hash,
            name: unsafe { sdk::read_weapon_model_info_name(ptr) }.to_string(),
            model_name: unsafe { sdk::read_weapon_model_info_model_name(ptr) }.to_string(),
            model_hash,
            ammo_type_hash,
            ammo_type: unsafe { sdk::read_weapon_model_info_ammo_type(ptr) }.to_string(),
            ammo_model_name: unsafe { sdk::read_weapon_model_info_ammo_model_name(ptr) }
                .to_string(),
            ammo_model_hash,
            default_max_ammo_mp,
            skill_above_50_max_ammo_mp,
            max_skill_max_ammo_mp,
            bonus_max_ammo_mp,
        })
    }
}
