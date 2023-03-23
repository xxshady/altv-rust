pub use core_shared::*;

#[derive(Debug)]
pub struct ResourceForModule {
    pub handlers: ResourceHandlers,
}

impl ResourceForModule {
    pub fn new(handlers: ResourceHandlers) -> Self {
        Self { handlers }
    }

    pub fn on_sdk_event(&self, event_type: altv_sdk::EventType, event: altv_sdk::CEventPtr) {
        self.handlers.on_sdk_event.as_ref().unwrap()(event_type, event);
    }

    pub fn on_tick(&self) {
        self.handlers.on_tick.as_ref().unwrap()();
    }

    pub fn on_base_object_create(
        &self,
        base_object: altv_sdk::IBaseObjectMutPtr,
        base_object_type: altv_sdk::BaseObjectType,
    ) {
        self.handlers.on_base_object_create.as_ref().unwrap()(base_object, base_object_type);
    }

    pub fn on_base_object_destroy(
        &self,
        base_object: altv_sdk::IBaseObjectMutPtr,
        _: altv_sdk::BaseObjectType,
    ) {
        self.handlers.on_base_object_destroy.as_ref().unwrap()(base_object);
    }
}
