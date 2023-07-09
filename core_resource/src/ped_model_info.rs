use crate::{
    helpers::{IntoHash, Hash},
    sdk,
};

#[derive(Debug)]
pub struct PedModelInfo {
    pub hash: Hash,
    pub name: String,
    pub model_type: String,
    pub dlc_name: String,
    pub default_unarmed_weapon: String,
    pub movement_clip_set: String,
}

impl PedModelInfo {
    pub fn get_by_hash(model: impl IntoHash) -> Option<Self> {
        let ptr = unsafe { sdk::ICore::GetPedModelByHash(model.into_hash()) };

        if !unsafe { sdk::is_ped_model_info_valid(ptr) } {
            return None;
        }

        Some(Self {
            hash: unsafe { sdk::read_ped_model_info_hash(ptr) },
            name: unsafe { sdk::read_ped_model_info_name(ptr) }.to_string(),
            model_type: unsafe { sdk::read_ped_model_info_type(ptr) }.to_string(),
            dlc_name: unsafe { sdk::read_ped_model_info_dlc_name(ptr) }.to_string(),
            default_unarmed_weapon: unsafe { sdk::read_ped_model_info_default_unarmed_weapon(ptr) }
                .to_string(),
            movement_clip_set: unsafe { sdk::read_ped_model_info_movement_clip_set(ptr) }
                .to_string(),
        })
    }
}
