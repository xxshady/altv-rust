use crate::{helpers::IntoHash, sdk, structs};

#[derive(Debug)]
pub struct VehicleModelInfo {
    pub title: String,
    pub model_type: altv_sdk::VehicleModelType,
    pub wheels_count: u8,
    pub has_armored_windows: bool,
    pub primary_color: u8,
    pub secondary_color: u8,
    pub pearl_color: u8,
    pub wheels_color: u8,
    pub interior_color: u8,
    pub dashboard_color: u8,
    pub mod_kits: [bool; 2],
    pub has_auto_attach_trailer: bool,
    pub bones: Vec<structs::BoneInfo>,

    pub ptr: *const sdk::alt::VehicleModelInfo,
}

impl VehicleModelInfo {
    pub fn get_by_hash(model: impl IntoHash) -> Option<Self> {
        let ptr = unsafe { sdk::ICore::GetVehicleModelByHash(model.into_hash()) };

        if !unsafe { sdk::is_vehicle_model_info_valid(ptr) } {
            return None;
        }

        let mut model_type = 0u8;
        let mut wheels_count = 0u8;
        let mut has_armored_windows = false;
        let mut primary_color = 0u8;
        let mut secondary_color = 0u8;
        let mut pearl_color = 0u8;
        let mut wheels_color = 0u8;
        let mut interior_color = 0u8;
        let mut dashboard_color = 0u8;
        let mut mod_kits = [false; 2];
        let mut has_auto_attach_trailer = false;

        unsafe {
            sdk::read_vehicle_model_info(
                ptr,
                &mut model_type as *mut u8,
                &mut wheels_count as *mut u8,
                &mut has_armored_windows as *mut bool,
                &mut primary_color as *mut u8,
                &mut secondary_color as *mut u8,
                &mut pearl_color as *mut u8,
                &mut wheels_color as *mut u8,
                &mut interior_color as *mut u8,
                &mut dashboard_color as *mut u8,
                &mut mod_kits as *mut bool,
                &mut has_auto_attach_trailer as *mut bool,
            )
        };
        let model_type = altv_sdk::VehicleModelType::from(model_type).unwrap();

        let title = unsafe { sdk::read_vehicle_model_info_title(ptr) }.to_string();

        let bones: Vec<structs::BoneInfo> = unsafe { sdk::read_vehicle_model_info_bones(ptr) }
            .into_iter()
            .map(|v| {
                let mut id = 0u16;
                let mut index = 0u16;
                unsafe { sdk::read_bone_info(v, &mut id as *mut u16, &mut index as *mut u16) }
                structs::BoneInfo {
                    id,
                    index,
                    name: unsafe { sdk::read_bone_info_name(v) }.to_string(),
                }
            })
            .collect();

        Some(Self {
            title,
            model_type,
            wheels_count,
            has_armored_windows,
            primary_color,
            secondary_color,
            pearl_color,
            wheels_color,
            interior_color,
            dashboard_color,
            mod_kits,
            has_auto_attach_trailer,
            bones,

            ptr,
        })
    }

    pub fn has_extra(&self, extra_id: u8) -> bool {
        unsafe { sdk::VehicleModelInfo::DoesExtraExist(self.ptr, extra_id) }
    }

    pub fn has_default_extra(&self, extra_id: u8) -> bool {
        unsafe { sdk::VehicleModelInfo::DoesExtraDefault(self.ptr, extra_id) }
    }
}
