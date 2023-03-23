use crate::{
    base_object_maps::BaseObjectMap, col_shape::ColShapeContainer, helpers::get_player_from_event,
    mvalue, player::PlayerContainer, resource::Resource, script_events::ScriptEventManager,
    vehicle::VehicleContainer,
};
use altv_sdk::ffi as sdk;
pub use altv_sdk::EventType as SDKEventType;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct ServerStartedController {}

#[derive(Debug)]
pub struct PlayerConnectController {
    pub player: PlayerContainer,
}

#[derive(Debug)]
pub struct PlayerDisconnectController {
    pub player: PlayerContainer,
    pub reason: String,
}

#[derive(Debug)]
pub struct BaseObjectCreateController {}

#[derive(Debug)]
pub struct ConsoleCommandController {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Debug)]
pub struct VehicleEnterColShapeController {
    pub vehicle: VehicleContainer,
    pub col_shape: ColShapeContainer,
}

#[repr(u16)]
pub enum Event {
    ServerStarted(Box<dyn FnMut(ServerStartedController)>),
    PlayerConnect(Box<dyn FnMut(PlayerConnectController)>),
    PlayerDisconnect(Box<dyn FnMut(PlayerDisconnectController)>),
    BaseObjectCreate(Box<dyn FnMut(BaseObjectCreateController)>),
    ConsoleCommand(Box<dyn FnMut(ConsoleCommandController)>),
    ResourceStop(fn()),
}

impl std::fmt::Debug for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO:
        write!(f, "Event <any>")
    }
}

#[derive(Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum ExtraEventType {
    VehicleEnterColShape,
}

pub enum ExtraEvent {
    VehicleEnterColShape(Box<dyn FnMut(VehicleEnterColShapeController)>),
}

impl std::fmt::Debug for ExtraEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO:
        write!(f, "ExtraEvent <any>")
    }
}

#[derive(Debug, Default)]
pub struct EventManager {
    public_handlers: HashMap<SDKEventType, Vec<Event>>,
    extra_public_handlers: HashMap<ExtraEventType, Vec<ExtraEvent>>,
    enabled_sdk_events: HashSet<SDKEventType>,
}

impl EventManager {
    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn on_sdk_event(
        &mut self,
        event_type: SDKEventType,
        event: *const sdk::alt::CEvent,
        resource: &Resource,
    ) {
        logger::debug!("received event: {:?}", event_type);

        // i'm just too retarded to make generic function...
        macro_rules! prepare_script_event_if_handled {
            ($manager: expr, $sdk_class: ident) => {{
                paste::paste! {
                    let converted_event = unsafe { sdk::events::[<to_ $sdk_class>](event) };
                    let event_name = unsafe { sdk::$sdk_class::GetName(converted_event) };
                    let event_name = event_name.as_ref().unwrap().to_string();
                    if $manager.is_event_handled(&event_name) {
                        let args = unsafe { sdk::$sdk_class::GetArgs(converted_event) };
                        let deserialized_args = mvalue::deserialize_mvalue_args(args, resource);
                        Some((event_name, deserialized_args))
                    } else {
                        logger::debug!("script event name: {event_name} ({event_type:?}) is unhandled");
                        None
                    }
                }
            }};
        }

        match event_type {
            SDKEventType::SERVER_SCRIPT_EVENT => {
                if let Some((event_name, deserialized_args)) = prepare_script_event_if_handled!(
                    resource.local_script_events.borrow(),
                    CServerScriptEvent
                ) {
                    resource
                        .local_script_events
                        .borrow_mut()
                        .handle_event(&event_name, &deserialized_args);
                }
            }
            SDKEventType::CLIENT_SCRIPT_EVENT => {
                if let Some((event_name, deserialized_args)) = prepare_script_event_if_handled!(
                    resource.client_script_events.borrow(),
                    CClientScriptEvent
                ) {
                    resource.client_script_events.borrow_mut().handle_event(
                        &event_name,
                        get_player_from_event(
                            unsafe { sdk::events::to_CClientScriptEvent(event) },
                            resource,
                            sdk::CClientScriptEvent::GetTarget,
                        ),
                        &deserialized_args,
                    );
                }
            }
            SDKEventType::COLSHAPE_EVENT => {
                let event = unsafe { sdk::events::to_CColShapeEvent(event) };
                let state = unsafe { sdk::CColShapeEvent::GetState(event) };

                let col_shape = unsafe { sdk::CColShapeEvent::GetTarget(event) };
                if col_shape.is_null() {
                    panic!("sdk::CColShapeEvent::GetTarget returned null");
                }

                let entity = unsafe { sdk::CColShapeEvent::GetEntity(event) };
                if entity.is_null() {
                    panic!("sdk::CColShapeEvent::GetEntity returned null");
                }

                let entity_base_type = unsafe {
                    altv_sdk::helpers::get_base_object_type(sdk::entity::to_base_object(entity))
                };

                match (state, entity_base_type) {
                    (true, altv_sdk::BaseObjectType::VEHICLE) => {
                        let vehicle = resource
                            .vehicle_base_object_map
                            .borrow()
                            .get_by_base_object_ptr(unsafe { sdk::entity::to_base_object(entity) })
                            .unwrap();
                        let col_shape = resource
                            .col_shape_base_object_map
                            .borrow()
                            .get_by_base_object_ptr(unsafe {
                                sdk::col_shape::to_base_object(col_shape)
                            })
                            .unwrap();

                        let extra_event = &ExtraEventType::VehicleEnterColShape;

                        if let Some(handlers) = self.extra_public_handlers.get_mut(extra_event) {
                            for h in handlers {
                                match h {
                                    ExtraEvent::VehicleEnterColShape(callback) => {
                                        callback(VehicleEnterColShapeController {
                                            vehicle: vehicle.clone(),
                                            col_shape: col_shape.clone(),
                                        })
                                    }
                                }
                            }
                        } else {
                            logger::debug!(
                                "[on_sdk_event] received extra event without handlers: {extra_event:?}",
                            );
                        }
                    }
                    _ => {
                        // TODO:
                        panic!("unknown col_shape event params: state: {state:?} entity_base_type: {entity_base_type:?}");
                    }
                }
            }
            _ => {
                let handlers = self.public_handlers.get_mut(&event_type);

                if let Some(handlers) = handlers {
                    // TODO: get event payload before loop for better performance

                    for h in handlers {
                        use Event::*;
                        match h {
                            ServerStarted(callback) => callback(ServerStartedController {}),
                            PlayerConnect(callback) => callback(PlayerConnectController {
                                player: get_player_from_event(
                                    unsafe { sdk::events::to_CPlayerConnectEvent(event) },
                                    resource,
                                    sdk::CPlayerConnectEvent::GetTarget,
                                ),
                            }),
                            PlayerDisconnect(callback) => callback(PlayerDisconnectController {
                                player: get_player_from_event(
                                    unsafe { sdk::events::to_CPlayerDisconnectEvent(event) },
                                    resource,
                                    sdk::CPlayerDisconnectEvent::GetTarget,
                                ),
                                reason: unsafe {
                                    sdk::CPlayerDisconnectEvent::GetReason(
                                        sdk::events::to_CPlayerDisconnectEvent(event),
                                    )
                                }
                                .to_string(),
                            }),
                            ConsoleCommand(callback) => callback({
                                let event = unsafe { sdk::events::to_CConsoleCommandEvent(event) };
                                ConsoleCommandController {
                                    name: unsafe { sdk::CConsoleCommandEvent::GetName(event) }
                                        .to_string(),
                                    args: unsafe { sdk::CConsoleCommandEvent::GetArgs(event) }
                                        .iter()
                                        .map(|s| s.to_string())
                                        .collect(),
                                }
                            }),
                            event => panic!("unknown event: {event:?}"),
                        }
                    }
                } else {
                    logger::debug!(
                        "[on_sdk_event] received event without handlers: {event_type:?}",
                    );
                }
            }
        }
    }

    pub fn add_handler(&mut self, sdk_type: SDKEventType, event: Event) {
        logger::debug!("events add_handler event sdk_type: {sdk_type:?}");

        if sdk_type != SDKEventType::NONE && self.enabled_sdk_events.get(&sdk_type).is_none() {
            logger::debug!("sdk_type: {sdk_type:?} | enabling it in sdk");

            self.enabled_sdk_events.insert(sdk_type);
            unsafe {
                sdk::ICore::ToggleEvent(sdk_type as u16, true);
            }
        }

        let handlers = self.public_handlers.entry(sdk_type).or_insert(vec![]);
        handlers.push(event);
    }

    pub fn add_extra_handler(
        &mut self,
        public_event: Event,
        extra_type: ExtraEventType,
        sdk_type: SDKEventType,
        event: ExtraEvent,
    ) {
        logger::debug!("events add_extra_handler event extra_type: {extra_type:?} public_event: {public_event:?}");

        if sdk_type != SDKEventType::NONE && self.enabled_sdk_events.get(&sdk_type).is_none() {
            logger::debug!("sdk_type: {sdk_type:?} | enabling it in sdk");

            self.enabled_sdk_events.insert(sdk_type);
            unsafe {
                sdk::ICore::ToggleEvent(sdk_type as u16, true);
            }
        }

        let handlers = self
            .extra_public_handlers
            .entry(extra_type)
            .or_insert(vec![]);
        handlers.push(event);

        self.add_handler(sdk_type, public_event);
    }
}

pub fn add_handler(sdk_type: SDKEventType, event: Event) {
    Resource::with_events_mut(|mut events, _| {
        events.add_handler(sdk_type, event);
    });
}

pub fn add_extra_handler(
    public_event: Event,
    extra_type: ExtraEventType,
    sdk_type: SDKEventType,
    event: ExtraEvent,
) {
    Resource::with_events_mut(|mut events, _| {
        events.add_extra_handler(public_event, extra_type, sdk_type, event);
    });
}
