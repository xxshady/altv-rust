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
    pub can_attach_cars: bool,

    ptr: *const sdk::alt::VehicleModelInfo,
}

impl VehicleModelInfo {
    pub fn get_by_hash(model: impl IntoHash) -> Option<Self> {
        let ptr = unsafe { sdk::ICore::GetVehicleModelByHash(model.into_hash()) };

        if !unsafe { sdk::is_vehicle_model_info_valid(ptr) } {
            return None;
        }

        let (
            mut model_type,
            mut wheels_count,
            mut has_armored_windows,
            mut primary_color,
            mut secondary_color,
            mut pearl_color,
            mut wheels_color,
            mut interior_color,
            mut dashboard_color,
            mut mod_kits, // [bool; 2], https://github.com/rust-lang/rust-analyzer/issues/15246
            mut has_auto_attach_trailer,
            mut can_attach_cars,
        ) = Default::default();

        unsafe {
            sdk::read_vehicle_model_info(
                ptr,
                &mut model_type,
                &mut wheels_count,
                &mut has_armored_windows,
                &mut primary_color,
                &mut secondary_color,
                &mut pearl_color,
                &mut wheels_color,
                &mut interior_color,
                &mut dashboard_color,
                &mut mod_kits as *mut bool,
                &mut has_auto_attach_trailer,
                &mut can_attach_cars,
            )
        };
        let model_type = altv_sdk::VehicleModelType::try_from(model_type).unwrap();

        let title = unsafe { sdk::read_vehicle_model_info_title(ptr) }.to_string();

        let bones: Vec<structs::BoneInfo> = unsafe { sdk::read_vehicle_model_info_bones(ptr) }
            .into_iter()
            .map(|v| {
                let (mut id, mut index) = Default::default();
                unsafe { sdk::read_bone_info(v, &mut id, &mut index) }
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
            can_attach_cars,

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
