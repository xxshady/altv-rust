use crate::{helpers::IntoHash, sdk};

#[derive(Debug)]
pub struct WeaponModelInfo {
    pub hash: u32,
    pub name: String,
    pub ammo_type: String,
    pub model_hash: u32,
    pub ammo_type_hash: u32,
    pub ammo_model_hash: u32,
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

        let mut hash = 0u32;
        let mut model_hash = 0u32;
        let mut ammo_type_hash = 0u32;
        let mut ammo_model_hash = 0u32;
        let mut default_max_ammo_mp = 0i32;
        let mut skill_above_50_max_ammo_mp = 0i32;
        let mut max_skill_max_ammo_mp = 0i32;
        let mut bonus_max_ammo_mp = 0i32;

        unsafe {
            sdk::read_weapon_model_info(
                ptr,
                &mut hash as *mut _,
                &mut model_hash as *mut _,
                &mut ammo_type_hash as *mut _,
                &mut ammo_model_hash as *mut _,
                &mut default_max_ammo_mp as *mut _,
                &mut skill_above_50_max_ammo_mp as *mut _,
                &mut max_skill_max_ammo_mp as *mut _,
                &mut bonus_max_ammo_mp as *mut _,
            )
        }

        Some(Self {
            hash,
            name: unsafe { sdk::read_weapon_model_info_name(ptr) }.to_string(),
            ammo_type: unsafe { sdk::read_weapon_model_info_ammo_type(ptr) }.to_string(),
            model_hash,
            ammo_type_hash,
            ammo_model_hash,
            default_max_ammo_mp,
            skill_above_50_max_ammo_mp,
            max_skill_max_ammo_mp,
            bonus_max_ammo_mp,
        })
    }
}
