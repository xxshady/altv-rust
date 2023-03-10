use std::fmt::Debug;

#[derive(Default)]
pub struct ResourceHandlers {
    pub on_tick: Option<Box<dyn Fn() + 'static>>,
    pub on_sdk_event: Option<Box<dyn Fn(altv_sdk::EventType, altv_sdk::CEventPtr) + 'static>>,
    pub on_base_object_create:
        Option<Box<dyn Fn(altv_sdk::IBaseObjectMutPtr, altv_sdk::BaseObjectType) + 'static>>,
    pub on_base_object_destroy: Option<Box<dyn Fn(altv_sdk::IBaseObjectMutPtr) + 'static>>,
}

impl ResourceHandlers {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Debug for ResourceHandlers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceHandlers {{todo}}")
    }
}
