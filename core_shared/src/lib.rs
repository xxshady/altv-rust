use std::fmt::Debug;

#[derive(Default)]
pub struct ResourceHandlers {
    pub on_tick: Option<Box<dyn Fn() + 'static>>,
    pub on_sdk_event: Option<Box<dyn Fn(altv_sdk::EventType, altv_sdk::CEventPtr) + 'static>>,
    pub on_base_object_create:
        Option<Box<dyn Fn(altv_sdk::BaseObjectMutPtr, altv_sdk::BaseObjectType) + 'static>>,
    pub on_base_object_destroy:
        Option<Box<dyn Fn(altv_sdk::BaseObjectMutPtr, altv_sdk::BaseObjectType) + 'static>>,
}

impl Debug for ResourceHandlers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceHandlers {{todo}}")
    }
}

pub type ResourceName = String;
type ToggleEventTypeFn = fn(ResourceName, altv_sdk::EventType, bool);

pub struct ModuleHandlers {
    pub toggle_event_type: ToggleEventTypeFn,
}

// this shit is here for derive(Default) of core_altv Resource
impl Default for ModuleHandlers {
    fn default() -> Self {
        fn placeholder(_: ResourceName, _: altv_sdk::EventType, _: bool) {}
        Self {
            toggle_event_type: placeholder,
        }
    }
}

impl ModuleHandlers {
    pub fn new(toggle_event_type: ToggleEventTypeFn) -> Self {
        Self { toggle_event_type }
    }
}

impl Debug for ModuleHandlers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ResourceHandlers {{todo}}")
    }
}
