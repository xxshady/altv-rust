use std::{cell::RefCell, ptr::NonNull};

use altv_sdk::ffi as sdk;
use autocxx::prelude::*;
use lazycell::LazyCell;

use crate::{
    base_objects::{col_shape, player},
    exports::{AnyEntity, Vector3},
    helpers::{get_entity_from_event, get_player_from_event, read_cpp_vector3, Hash},
    mvalue,
    resource::Resource,
    VoidResult,
};

#[derive(Debug)]
pub struct PlayerConnect {
    pub player: player::PlayerContainer,
}

impl PlayerConnect {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        Self {
            player: get_player_from_event(
                sdk::events::to_CPlayerConnectEvent(event),
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
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = sdk::events::to_CPlayerDisconnectEvent(event);
        Self {
            player: get_player_from_event(event, resource, sdk::CPlayerDisconnectEvent::GetTarget),
            reason: sdk::CPlayerDisconnectEvent::GetReason(event).to_string(),
        }
    }
}

#[derive(Debug)]
pub struct ServerStarted {}

impl ServerStarted {
    pub(crate) unsafe fn new(_: altv_sdk::CEventPtr, _: &Resource) -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct ColshapeEvent {
    pub col_shape: col_shape::ColShapeMutPtr,
    pub entity: AnyEntity,
    pub state: bool,
}

impl ColshapeEvent {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = sdk::events::to_CColShapeEvent(event);

        let state = sdk::CColShapeEvent::GetState(event);

        let col_shape = sdk::CColShapeEvent::GetTarget(event);
        let col_shape = NonNull::new(col_shape).unwrap();

        Self {
            col_shape,
            entity: get_entity_from_event(event, resource, sdk::CColShapeEvent::GetEntity),
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
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, _: &Resource) -> Self {
        let event = sdk::events::to_CServerScriptEvent(event);
        let name = sdk::CServerScriptEvent::GetName(event).to_string();
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
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = sdk::events::to_CClientScriptEvent(event);
        let name = sdk::CClientScriptEvent::GetName(event).to_string();
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
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, _: &Resource) -> Self {
        let event = sdk::events::to_CConsoleCommandEvent(event);
        Self {
            name: sdk::CConsoleCommandEvent::GetName(event).to_string(),
            args: sdk::CConsoleCommandEvent::GetArgs(event)
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct WeaponDamageEvent {
    pub source: player::PlayerContainer,
    pub target: AnyEntity,
    pub weapon_hash: Hash,
    pub body_part: altv_sdk::PlayerBodyPart,
    pub damage: u32,
    pub shot_offset: Vector3,

    event: *mut sdk::alt::CWeaponDamageEvent,
    base_event: altv_sdk::CEventPtr,
    custom_damage_set: RefCell<bool>,
}

impl WeaponDamageEvent {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let weapon_event = sdk::events::to_CWeaponDamageEvent(event);
        let weapon_event = NonNull::new(weapon_event).unwrap();
        let weapon_event = weapon_event.as_ptr();

        Self {
            source: get_player_from_event(
                weapon_event,
                resource,
                sdk::CWeaponDamageEvent::GetSource,
            ),
            target: get_entity_from_event(
                weapon_event,
                resource,
                sdk::CWeaponDamageEvent::GetTarget,
            ),
            weapon_hash: sdk::CWeaponDamageEvent::GetWeaponHash(weapon_event),
            body_part: {
                let raw = sdk::CWeaponDamageEvent::GetBodyPart(weapon_event);
                altv_sdk::PlayerBodyPart::from(raw).unwrap()
            },
            damage: sdk::CWeaponDamageEvent::GetDamageValue(weapon_event),
            shot_offset: {
                let raw = sdk::CWeaponDamageEvent::GetShotOffset(weapon_event).within_unique_ptr();
                read_cpp_vector3(raw)
            },

            // internal properties
            base_event: event,
            event: weapon_event,
            custom_damage_set: RefCell::new(false),
        }
    }

    pub fn set_damage(&self, value: u32) -> VoidResult {
        if *self.custom_damage_set.try_borrow()? {
            anyhow::bail!("Damage cannot be set multiple times")
        }
        *self.custom_damage_set.try_borrow_mut()? = true;
        unsafe { sdk::CWeaponDamageEvent::SetDamageValue(self.event, value) }
        Ok(())
    }

    pub fn cancel(&self) -> VoidResult {
        if unsafe { sdk::CEvent::WasCancelled(self.base_event) } {
            anyhow::bail!("Event cannot be cancelled multiple times")
        } else {
            unsafe { sdk::CEvent::Cancel(self.base_event) }
            Ok(())
        }
    }
}
