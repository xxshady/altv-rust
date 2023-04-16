use altv_sdk::{ffi::ICore::ToggleEvent, EventType};
use std::collections::HashSet;

// these events are required for internal stuff of core_altv
lazy_static::lazy_static! {
    static ref REQUIRED_SDK_EVENTS: HashSet<EventType> = {
        HashSet::from([
            EventType::ResourceStart,
            EventType::ResourceStop,
            EventType::ConnectionQueueAdd,
            EventType::ConnectionQueueRemove,
        ])
    };
}

pub unsafe fn enable() {
    for event in REQUIRED_SDK_EVENTS.iter() {
        ToggleEvent(*event as u16, true);
    }
}

pub fn is_required(event_type: EventType) -> bool {
    REQUIRED_SDK_EVENTS.contains(&event_type)
}
