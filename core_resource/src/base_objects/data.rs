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

pub struct ColShapy<Data: Default> {
    pub(crate) col_shape_type: LazyCell<altv_sdk::ColShapeType>,

    // TODO: should be used by checkpoint later
    #[allow(dead_code)]
    pub(crate) data: Data,
}

impl<Data: Default> Default for ColShapy<Data> {
    fn default() -> Self {
        Self {
            col_shape_type: LazyCell::new(),
            data: Data::default(),
        }
    }
}

pub struct Blip {
    pub(crate) blip_type: LazyCell<altv_sdk::BlipType>,
}

impl Default for Blip {
    fn default() -> Self {
        Self {
            blip_type: LazyCell::new(),
        }
    }
}
