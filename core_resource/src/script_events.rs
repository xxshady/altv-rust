use std::{
    collections::{HashMap, hash_map},
    fmt::Debug,
    marker::PhantomData,
    cell::RefMut,
};

use crate::{
    base_objects::player, helpers::IntoString, resource::Resource, IntoVoidResult, VoidResult,
    SomeResult,
};
use altv_sdk::ffi as sdk;
use anyhow::bail;

pub fn emit(event_name: impl IntoString, args: mvalue::DynMValueArgs) -> VoidResult {
    unsafe { sdk::trigger_local_event(event_name.into_string(), mvalue::serialize_args(args)?) };
    Ok(())
}

pub type EventArgs<'a> = &'a Vec<mvalue::ConstMValue>;
pub type HandlerId = u64;
type EventName = String;

#[derive(Debug)]
pub struct LocalEventContext<'a> {
    pub args: EventArgs<'a>,
}

#[derive(Debug)]
pub struct PlayerEventContext<'a> {
    pub player: player::PlayerContainer,
    pub args: EventArgs<'a>,
}

pub type LocalEventHandler = Box<dyn FnMut(&LocalEventContext) -> VoidResult>;
pub type ClientEventHandler = Box<dyn FnMut(&PlayerEventContext) -> VoidResult>;

pub trait ScriptEventManager {
    type Handler;

    fn handlers(&self) -> &HashMap<EventName, HashMap<HandlerId, Self::Handler>>;
    fn handlers_mut(&mut self) -> &mut HashMap<EventName, HashMap<HandlerId, Self::Handler>>;

    fn next_handler_id(&mut self) -> HandlerId;

    fn is_event_handled(&self, event_name: &str) -> bool {
        self.handlers().contains_key(event_name)
    }

    fn add_handler(&mut self, event_name: EventName, handler: Self::Handler) -> HandlerId {
        let handler_id = self.next_handler_id();

        self.handlers_mut()
            .entry(event_name)
            .or_insert(HashMap::new())
            .insert(handler_id, handler);

        handler_id
    }

    fn remove_handler(&mut self, event_name: &str, handler_id: HandlerId) -> VoidResult {
        let handlers = self.handlers_mut().get_mut(event_name);
        let Some(handlers) = handlers else {
            bail!("No handlers for event: {event_name}")
        };

        let Some(_) = handlers.remove(&handler_id) else {
            bail!("Handler was not found in the map")
        };

        if !handlers.is_empty() {
            return Ok(());
        }

        let Some(_) = self.handlers_mut().remove(event_name) else {
            bail!("Handlers map of event: {event_name} was not found in the map (handlers map should be removed because its empty)")
        };

        Ok(())
    }

    fn get_handlers_for_event(
        &mut self,
        event_name: &str,
    ) -> Option<hash_map::ValuesMut<HandlerId, Self::Handler>> {
        self.handlers_mut()
            .get_mut(event_name)
            .map(|v| v.values_mut())
    }

    fn remove_scheduled_handlers<T>(&mut self, mut schedule: RefMut<EventSchedule<T>>) {
        for (event_name, id) in schedule.to_be_destroyed.iter() {
            let result = self.remove_handler(event_name, *id);

            match result {
                Ok(_) => {
                    logger::debug!(
                        "script event handler of event: \"{event_name}\" removed successfully"
                    );
                }
                Err(e) => {
                    logger::error!("Failed to remove script event handler (local or client) of event: \"{event_name}\" with id: {id} (error: {e})");
                }
            }
        }
        schedule.to_be_destroyed.clear();
    }
}

#[derive(Default)]
pub struct LocalEventManager {
    handlers: HashMap<EventName, HashMap<HandlerId, LocalEventHandler>>,
    handler_id: HandlerId,
}

impl LocalEventManager {
    pub fn init() {
        use crate::events;

        events::add_sdk_handler(events::SDKHandler::ServerScriptEvent(Box::new(|c| {
            let events::sdk_contexts::ServerScriptEvent { name, args } = c;

            Resource::with_local_script_events_mut(|mut events, resource| {
                if !events.is_event_handled(name) {
                    logger::debug!("local event is unhandled: {name}");
                    return;
                }
                events.handle_event(
                    resource.local_script_events_schedule.borrow_mut(),
                    name,
                    args,
                );
            });

            Ok(())
        })));
    }

    fn handle_event(
        &mut self,
        schedule: RefMut<LocalEventSchedule>,
        event_name: &str,
        args: EventArgs,
    ) {
        self.remove_scheduled_handlers(schedule);

        if let Some(handlers) = self.get_handlers_for_event(event_name) {
            let context = LocalEventContext { args };
            for h in handlers {
                if let Err(error) = h(&context) {
                    logger::error!("handler of event: {event_name:?} failed with error: {error:?}");
                } else {
                    logger::debug!("handler of event: {event_name:?} called successfully");
                }
            }
        } else {
            logger::debug!("handle_event no handlers for local event: {event_name:?}")
        }
    }
}

impl ScriptEventManager for LocalEventManager {
    type Handler = LocalEventHandler;

    fn handlers(&self) -> &HashMap<EventName, HashMap<HandlerId, Self::Handler>> {
        &self.handlers
    }

    fn handlers_mut(&mut self) -> &mut HashMap<EventName, HashMap<HandlerId, Self::Handler>> {
        &mut self.handlers
    }

    fn next_handler_id(&mut self) -> HandlerId {
        self.handler_id += 1;
        self.handler_id
    }
}

impl Debug for LocalEventManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocalEventManager{{todo}}")
    }
}

#[derive(Default)]
pub struct ClientEventManager {
    handlers: HashMap<EventName, HashMap<HandlerId, ClientEventHandler>>,
    handler_id: HandlerId,
}

impl ClientEventManager {
    pub fn init() {
        use crate::events;

        events::add_sdk_handler(events::SDKHandler::ClientScriptEvent(Box::new(|c| {
            let events::sdk_contexts::ClientScriptEvent { name, player, args } = c;

            Resource::with_client_script_events_mut(|mut events, resource| {
                if !events.is_event_handled(name) {
                    logger::debug!("client event is unhandled: {name}");
                    return;
                }
                events.handle_event(
                    resource.client_script_events_schedule.borrow_mut(),
                    name,
                    player.clone(),
                    args,
                );
            });

            Ok(())
        })));
    }

    pub fn handle_event(
        &mut self,
        schedule: RefMut<ClientEventSchedule>,
        event_name: &str,
        player: player::PlayerContainer,
        args: EventArgs,
    ) {
        self.remove_scheduled_handlers(schedule);

        if let Some(handlers) = self.get_handlers_for_event(event_name) {
            let context = PlayerEventContext { player, args };
            for h in handlers {
                if let Err(error) = h(&context) {
                    logger::error!(
                        "handler of client event: {event_name:?} failed with error: {error:?}"
                    );
                } else {
                    logger::debug!("handler of client event: {event_name:?} called successfully");
                }
            }
        } else {
            logger::debug!("handle_event no handlers for client event: {event_name:?}")
        }
    }
}

impl ScriptEventManager for ClientEventManager {
    type Handler = ClientEventHandler;

    fn handlers(&self) -> &HashMap<EventName, HashMap<HandlerId, Self::Handler>> {
        &self.handlers
    }

    fn handlers_mut(&mut self) -> &mut HashMap<EventName, HashMap<HandlerId, Self::Handler>> {
        &mut self.handlers
    }

    fn next_handler_id(&mut self) -> HandlerId {
        self.handler_id += 1;
        self.handler_id
    }
}

impl Debug for ClientEventManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ClientEventManager {{todo}} ")
    }
}

pub fn on<V: IntoVoidResult>(
    event_name: impl IntoString,
    mut handler: impl FnMut(&LocalEventContext) -> V + 'static,
) -> LocalEventController {
    let event_name = event_name.into_string();

    let id = Resource::with_local_script_events_mut(|mut local_events, _| {
        local_events.add_handler(
            event_name.clone(),
            Box::new(move |c| handler(c).into_void_result()),
        )
    });

    LocalEventController::new(event_name, id)
}

pub fn on_player<V: IntoVoidResult>(
    event_name: impl IntoString,
    mut handler: impl FnMut(&PlayerEventContext) -> V + 'static,
) -> PlayerEventController {
    let event_name = event_name.into_string();

    let id = Resource::with_client_script_events_mut(|mut local_events, _| {
        local_events.add_handler(
            event_name.clone(),
            Box::new(move |c| handler(c).into_void_result()),
        )
    });

    PlayerEventController::new(event_name, id)
}

pub mod script_event_type {
    #[derive(Debug, Default)]
    pub struct Local;
    #[derive(Debug, Default)]
    pub struct Client;
}

#[derive(Debug)]
pub struct ScriptEventController<T> {
    event_name: Option<EventName>,
    id: Option<HandlerId>,
    __type: PhantomData<T>,
}

impl<T> ScriptEventController<T> {
    fn new(event_name: EventName, id: HandlerId) -> Self {
        Self {
            event_name: Some(event_name),
            id: Some(id),
            __type: PhantomData,
        }
    }

    fn take_data(&mut self) -> SomeResult<(EventName, HandlerId)> {
        let (Some(id), Some(event_name)) = (self.id.take(), self.event_name.take()) else {
            bail!("Already destroyed")
        };
        Ok((event_name, id))
    }
}

pub type LocalEventController = ScriptEventController<script_event_type::Local>;

impl LocalEventController {
    pub fn destroy(&mut self) -> VoidResult {
        let (event_name, id) = self.take_data()?;

        Resource::with_local_script_events_schedule_mut(|mut v, _| {
            v.add_to_be_destroyed(event_name, id)
        });

        Ok(())
    }
}

pub type PlayerEventController = ScriptEventController<script_event_type::Client>;

impl PlayerEventController {
    pub fn destroy(&mut self) -> VoidResult {
        let (event_name, id) = self.take_data()?;

        Resource::with_client_script_events_schedule_mut(|mut v, _| {
            v.add_to_be_destroyed(event_name, id)
        });

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct EventSchedule<T> {
    to_be_destroyed: Vec<(EventName, HandlerId)>,
    __type: PhantomData<T>,
}

impl<T> EventSchedule<T> {
    fn add_to_be_destroyed(&mut self, event_name: EventName, handler_id: HandlerId) {
        self.to_be_destroyed.push((event_name, handler_id));
    }
}

pub type LocalEventSchedule = EventSchedule<script_event_type::Local>;
pub type ClientEventSchedule = EventSchedule<script_event_type::Client>;
