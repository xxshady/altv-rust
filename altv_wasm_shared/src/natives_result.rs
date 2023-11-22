#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOf_get_dlc_weapon_data {
    pub success: bool,
    pub ret: bool,
    pub outData_: shared::MemoryBufferId
}
