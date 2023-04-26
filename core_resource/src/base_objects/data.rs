use lazycell::LazyCell;

pub struct Marker {
    pub(crate) marker_type: LazyCell<altv_sdk::MarkerType>,
}

impl Default for Marker {
    fn default() -> Self {
        Self {
            marker_type: LazyCell::new(),
        }
    }
}
