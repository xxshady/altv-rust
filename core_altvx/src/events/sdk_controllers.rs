use std::ptr::NonNull;

use altv_sdk::ffi as sdk;
use lazycell::LazyCell;

use crate::{
    base_objects::{col_shape, player},
    helpers::get_player_from_event,
    mvalue,
    resource::Resource,
};

#[derive(Debug)]
pub struct PlayerConnect {
    pub player: player::PlayerContainer,
}

impl PlayerConnect {
    pub fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        Self {
            player: get_player_from_event(
                unsafe { sdk::events::to_CPlayerConnectEvent(event) },
                resource,
                sdk::CPlayerConnectEvent::GetTarget,
            ),
        }
    }
}

#[derive(Debug)]
pub struct PlayerDisconnect {
    pub player: player::PlayerContainer,
    pub reason: String,
}

impl PlayerDisconnect {
    pub fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = unsafe { sdk::events::to_CPlayerDisconnectEvent(event) };
        Self {
            player: get_player_from_event(event, resource, sdk::CPlayerDisconnectEvent::GetTarget),
            reason: unsafe { sdk::CPlayerDisconnectEvent::GetReason(event) }.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ServerStarted {}

impl ServerStarted {
    pub fn new(_: altv_sdk::CEventPtr, _: &Resource) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct ColshapeEvent {
    pub col_shape: col_shape::ColShapeMutPtr,
    pub entity: altv_sdk::BaseObjectMutPtr,
    pub state: bool,
}

impl ColshapeEvent {
    pub fn new(event: altv_sdk::CEventPtr, _: &Resource) -> Self {
        let event = unsafe { sdk::events::to_CColShapeEvent(event) };

        let state = unsafe { sdk::CColShapeEvent::GetState(event) };

        let col_shape = unsafe { sdk::CColShapeEvent::GetTarget(event) };
        let col_shape = NonNull::new(col_shape).unwrap();

        let entity = unsafe { sdk::CColShapeEvent::GetEntity(event) };
        let entity = NonNull::new(entity).unwrap();
        let entity = unsafe { sdk::entity::to_base_object(entity.as_ptr()) };
        let entity = NonNull::new(entity).unwrap();

        Self {
            col_shape,
            entity,
            state,
        }
    }
}

#[derive(Debug)]
pub struct ServerScriptEvent {
    pub name: String,
    event: *const sdk::alt::CServerScriptEvent,
    args: LazyCell<mvalue::MValueList>,
}

impl ServerScriptEvent {
    pub fn new(event: altv_sdk::CEventPtr, _: &Resource) -> Self {
        let event = unsafe { sdk::events::to_CServerScriptEvent(event) };
        let name = unsafe { sdk::CServerScriptEvent::GetName(event) }.to_string();
        Self {
            name,
            event,
            args: LazyCell::new(),
        }
    }

    pub fn args(&self) -> &mvalue::MValueList {
        if self.args.filled() {
            self.args.borrow().unwrap()
        } else {
            let args = unsafe { sdk::CServerScriptEvent::GetArgs(self.event) };
            self.args
                .fill(Resource::with(|v| mvalue::deserialize_mvalue_args(args, v)))
                .unwrap();
            self.args.borrow().unwrap()
        }
    }
}

#[derive(Debug)]
pub struct ClientScriptEvent {
    pub name: String,
    pub player: player::PlayerContainer,
    event: *const sdk::alt::CClientScriptEvent,
    args: LazyCell<mvalue::MValueList>,
}

impl ClientScriptEvent {
    pub fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = unsafe { sdk::events::to_CClientScriptEvent(event) };
        let name = unsafe { sdk::CClientScriptEvent::GetName(event) }.to_string();
        let player = get_player_from_event(event, resource, sdk::CClientScriptEvent::GetTarget);

        Self {
            name,
            event,
            player,
            args: LazyCell::new(),
        }
    }

    pub fn args(&self) -> &mvalue::MValueList {
        if self.args.filled() {
            self.args.borrow().unwrap()
        } else {
            let args = unsafe { sdk::CClientScriptEvent::GetArgs(self.event) };
            self.args
                .fill(Resource::with(|v| mvalue::deserialize_mvalue_args(args, v)))
                .unwrap();
            self.args.borrow().unwrap()
        }
    }
}

#[derive(Debug)]
pub struct ConsoleCommandEvent {
    pub name: String,
    pub args: Vec<String>,
}

impl ConsoleCommandEvent {
    pub fn new(event: altv_sdk::CEventPtr, _: &Resource) -> Self {
        let event = unsafe { sdk::events::to_CConsoleCommandEvent(event) };
        Self {
            name: unsafe { sdk::CConsoleCommandEvent::GetName(event) }.to_string(),
            args: unsafe { sdk::CConsoleCommandEvent::GetArgs(event) }
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}
