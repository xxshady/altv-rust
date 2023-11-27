#[derive(serde::Serialize, serde::Deserialize)]
pub struct ResultOfGetDlcWeaponData {
    pub success: bool,
    pub ret: bool,
    pub out_data_: shared::MemoryBufferId
}
