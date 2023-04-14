use altv_sdk::ffi as sdk;
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use core_module::ResourceName;

use crate::required_sdk_events;

thread_local! {
    pub static EVENT_MANAGER_INSTANCE: RefCell<EventManager> = RefCell::new(EventManager::default());
}

type NumberOfListeners = u32;

#[derive(Debug, Default)]
pub struct EventManager {
    enabled_events: HashMap<ResourceName, HashSet<altv_sdk::EventType>>,
    event_listeners: HashMap<altv_sdk::EventType, NumberOfListeners>,
}

impl EventManager {
    pub fn toggle_event(
        &mut self,
        resource: ResourceName,
        event_type: altv_sdk::EventType,
        state: bool,
    ) {
        logger::debug!("toggle_event {event_type:?} {state} (resource: {resource})");
        if state {
            let events_of_resource = self.enabled_events.entry(resource).or_default();
            if !events_of_resource.insert(event_type) {
                return;
            }
            self.update_number_of_listeners(event_type, true);
        } else if let Some(events_of_resource) = self.enabled_events.get_mut(&resource) {
            if events_of_resource.remove(&event_type) {
                self.update_number_of_listeners(event_type, false);
            } else {
                logger::warn!("events_of_resource did not contain this event: {event_type:?}");
            }
        } else {
            logger::warn!("enabled_events did not contain events of this resource");
        }
    }

    pub fn resource_stopped(&mut self, resource: &ResourceName) {
        logger::debug!("resource_stopped {resource}");

        let events = self.enabled_events.remove(resource).unwrap_or_default();
        for event in events {
            self.update_number_of_listeners(event, false);
        }
    }

    fn update_number_of_listeners(&mut self, event_type: altv_sdk::EventType, increase: bool) {
        let number = self.event_listeners.entry(event_type).or_default();
        let event_u16 = event_type as u16;
        if increase {
            if *number == 0 {
                logger::debug!("enabling event");
                if !required_sdk_events::is_required(event_type) {
                    unsafe { sdk::ICore::ToggleEvent(event_u16, true) };
                } else {
                    logger::debug!("event is required, do nothing");
                }
            }
            *number += 1;
        } else {
            if *number == 1 {
                logger::debug!("disabling event: {event_type:?}");
                if !required_sdk_events::is_required(event_type) {
                    unsafe { sdk::ICore::ToggleEvent(event_u16, false) };
                } else {
                    logger::debug!("event is required, do nothing");
                }
            }
            *number -= 1;
        }
    }
}
