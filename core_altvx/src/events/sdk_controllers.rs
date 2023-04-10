use std::{cell::RefCell, ptr::NonNull, rc::Rc};

use altv_sdk::ffi as sdk;
use autocxx::prelude::*;
use lazycell::LazyCell;

use crate::{
    base_objects::{col_shape, player, vehicle},
    exports::{AnyEntity, Vector3},
    helpers::{read_cpp_vector3, Hash},
    mvalue,
    resource::Resource,
    VoidResult,
};

use super::{
    cancellable::CancellableEvent,
    connection_queue::{ConnectionQueueController, ConnectionQueueInfo},
    helpers::{
        base_event_to_specific, get_entity_from_event, get_non_null_entity_from_event,
        get_player_from_event, get_vehicle_from_event,
    },
    structs,
};

#[derive(Debug)]
pub struct PlayerConnect {
    pub player: player::PlayerContainer,
}

impl PlayerConnect {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(event, CPlayerConnectEvent);
        Self {
            player: get_player_from_event(sdk::CPlayerConnectEvent::GetTarget(event), resource),
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
            player: get_player_from_event(sdk::CPlayerDisconnectEvent::GetTarget(event), resource),
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
        let event = base_event_to_specific!(event, CColShapeEvent);

        let state = sdk::CColShapeEvent::GetState(event);

        let col_shape = sdk::CColShapeEvent::GetTarget(event);
        let col_shape = NonNull::new(col_shape).unwrap();

        Self {
            col_shape,
            entity: get_non_null_entity_from_event(event, resource, sdk::CColShapeEvent::GetEntity),
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
        let event = base_event_to_specific!(event, CServerScriptEvent);
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
        let event = base_event_to_specific!(event, CClientScriptEvent);
        let name = sdk::CClientScriptEvent::GetName(event).to_string();
        let player = get_player_from_event(sdk::CClientScriptEvent::GetTarget(event), resource);

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
    cancellable: CancellableEvent,
    custom_damage_set: RefCell<bool>,
}

impl WeaponDamageEvent {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let weapon_event = base_event_to_specific!(event, CWeaponDamageEvent);

        Self {
            source: get_player_from_event(
                sdk::CWeaponDamageEvent::GetSource(weapon_event),
                resource,
            ),
            target: get_non_null_entity_from_event(
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
            cancellable: CancellableEvent::new(event),
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
        self.cancellable.cancel()
    }
}

#[derive(Debug)]
pub struct PlayerDeath {
    pub player: player::PlayerContainer,
    pub killer: Option<AnyEntity>,
    pub weapon_hash: Hash,
}

impl PlayerDeath {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(event, CPlayerDeathEvent);

        Self {
            player: get_player_from_event(sdk::CPlayerDeathEvent::GetTarget(event), resource),
            killer: get_entity_from_event(sdk::CPlayerDeathEvent::GetKiller(event), resource),
            weapon_hash: sdk::CPlayerDeathEvent::GetWeapon(event),
        }
    }
}

#[derive(Debug)]
pub struct PlayerDamage {
    pub player: player::PlayerContainer,
    pub attacker: Option<AnyEntity>,
    pub health_damage: u16,
    pub armour_damage: u16,
}

impl PlayerDamage {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(event, CPlayerDamageEvent);

        Self {
            player: get_player_from_event(sdk::CPlayerDamageEvent::GetTarget(event), resource),
            attacker: get_entity_from_event(sdk::CPlayerDamageEvent::GetAttacker(event), resource),
            health_damage: sdk::CPlayerDamageEvent::GetHealthDamage(event),
            armour_damage: sdk::CPlayerDamageEvent::GetArmourDamage(event),
        }
    }
}

macro_rules! player_enter_or_leave_vehicle {
    ($event: ident) => {
        pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
            let event = base_event_to_specific!(event, $event);

            Self {
                player: get_player_from_event(sdk::$event::GetPlayer(event), resource),
                vehicle: get_vehicle_from_event(sdk::$event::GetTarget(event), resource),
                seat: sdk::$event::GetSeat(event),
            }
        }
    };
}

#[derive(Debug)]
pub struct PlayerEnteringVehicle {
    pub player: player::PlayerContainer,
    pub vehicle: vehicle::VehicleContainer,
    pub seat: u8,
}

impl PlayerEnteringVehicle {
    player_enter_or_leave_vehicle!(CPlayerEnteringVehicleEvent);
}

#[derive(Debug)]
pub struct PlayerEnterVehicle {
    pub player: player::PlayerContainer,
    pub vehicle: vehicle::VehicleContainer,
    pub seat: u8,
}

impl PlayerEnterVehicle {
    player_enter_or_leave_vehicle!(CPlayerEnterVehicleEvent);
}

#[derive(Debug)]
pub struct PlayerLeaveVehicle {
    pub player: player::PlayerContainer,
    pub vehicle: vehicle::VehicleContainer,
    pub seat: u8,
}

impl PlayerLeaveVehicle {
    player_enter_or_leave_vehicle!(CPlayerLeaveVehicleEvent);
}

#[derive(Debug)]
pub struct PlayerChangeVehicleSeat {
    pub player: player::PlayerContainer,
    pub vehicle: vehicle::VehicleContainer,
    pub new_seat: u8,
    pub old_seat: u8,
}

impl PlayerChangeVehicleSeat {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(event, CPlayerChangeVehicleSeatEvent);

        Self {
            player: get_player_from_event(
                sdk::CPlayerChangeVehicleSeatEvent::GetPlayer(event),
                resource,
            ),
            vehicle: get_vehicle_from_event(
                sdk::CPlayerChangeVehicleSeatEvent::GetTarget(event),
                resource,
            ),
            new_seat: sdk::CPlayerChangeVehicleSeatEvent::GetNewSeat(event),
            old_seat: sdk::CPlayerChangeVehicleSeatEvent::GetOldSeat(event),
        }
    }
}

#[derive(Debug)]
pub struct PlayerWeaponChange {
    pub player: player::PlayerContainer,
    pub new_weapon_hash: Hash,
    pub old_weapon_hash: Hash,
}

impl PlayerWeaponChange {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CPlayerWeaponChangeEvent);

        Self {
            player: get_player_from_event(
                sdk::CPlayerWeaponChangeEvent::GetTarget(event),
                resource,
            ),
            new_weapon_hash: sdk::CPlayerWeaponChangeEvent::GetNewWeapon(event),
            old_weapon_hash: sdk::CPlayerWeaponChangeEvent::GetOldWeapon(event),
        }
    }
}

#[derive(Debug)]
pub struct PlayerConnectDenied {
    pub reason: altv_sdk::PlayerConnectDeniedReason,
    pub name: String,
    pub ip: String,
    pub password_hash: u64,
    pub is_debug: bool,
    pub branch: String,
    pub major_version: u32,
    pub cdn_url: String,
    pub discord_id: i64,
}

impl PlayerConnectDenied {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, _: &Resource) -> Self {
        let event = base_event_to_specific!(event, CPlayerConnectDeniedEvent);

        use sdk::CPlayerConnectDeniedEvent::*;
        Self {
            reason: {
                let raw = GetReason(event);
                altv_sdk::PlayerConnectDeniedReason::from(raw).unwrap()
            },
            name: GetName(event).to_string(),
            ip: GetIp(event).to_string(),
            password_hash: GetPasswordHash(event),
            is_debug: IsDebug(event),
            branch: GetBranch(event).to_string(),
            major_version: GetMajorVersion(event),
            cdn_url: GetCdnUrl(event).to_string(),
            discord_id: GetDiscordId(event),
        }
    }
}

#[derive(Debug)]
pub struct PlayerSpawn {
    pub player: player::PlayerContainer,
}

impl PlayerSpawn {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(event, CPlayerSpawnEvent);

        Self {
            player: get_player_from_event(sdk::CPlayerSpawnEvent::GetPlayer(event), resource),
        }
    }
}

#[derive(Debug)]
pub struct PlayerRequestControl {
    pub player: player::PlayerContainer,
    pub entity: AnyEntity,
}

impl PlayerRequestControl {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(event, CPlayerRequestControlEvent);

        Self {
            player: get_player_from_event(
                sdk::CPlayerRequestControlEvent::GetPlayer(event),
                resource,
            ),
            entity: get_non_null_entity_from_event(
                event,
                resource,
                sdk::CPlayerRequestControlEvent::GetTarget,
            ),
        }
    }
}

#[derive(Debug)]
pub struct PlayerDimensionChange {
    pub player: player::PlayerContainer,
    pub new_dimension: i32,
    pub old_dimension: i32,
}

impl PlayerDimensionChange {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(event, CPlayerDimensionChangeEvent);

        Self {
            player: get_player_from_event(
                sdk::CPlayerDimensionChangeEvent::GetTarget(event),
                resource,
            ),
            new_dimension: sdk::CPlayerDimensionChangeEvent::GetNewDimension(event),
            old_dimension: sdk::CPlayerDimensionChangeEvent::GetOldDimension(event),
        }
    }
}

#[derive(Debug)]
pub struct PlayerChangeInteriorEvent {
    pub player: player::PlayerContainer,
    pub new_interior: u32,
    pub old_interior: u32,
}

impl PlayerChangeInteriorEvent {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(event, CPlayerChangeInteriorEvent);

        Self {
            player: get_player_from_event(
                sdk::CPlayerChangeInteriorEvent::GetTarget(event),
                resource,
            ),
            new_interior: sdk::CPlayerChangeInteriorEvent::GetNewInteriorLocation(event),
            old_interior: sdk::CPlayerChangeInteriorEvent::GetOldInteriorLocation(event),
        }
    }
}

#[derive(Debug)]
pub struct StartProjectileEvent {
    pub player: player::PlayerContainer,
    pub weapon_hash: Hash,
    pub ammo_hash: Hash,
    pub pos: Vector3,
    pub dir: Vector3,

    cancellable: CancellableEvent,
}

impl StartProjectileEvent {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CStartProjectileEvent);

        use sdk::CStartProjectileEvent::*;
        Self {
            player: get_player_from_event(GetSource(event), resource),
            weapon_hash: GetWeaponHash(event),
            ammo_hash: GetAmmoHash(event),
            pos: {
                let raw = GetStartPosition(event).within_unique_ptr();
                read_cpp_vector3(raw)
            },
            dir: {
                let raw = GetDirection(event).within_unique_ptr();
                read_cpp_vector3(raw)
            },

            cancellable: CancellableEvent::new(base_event),
        }
    }

    pub fn cancel(&self) -> VoidResult {
        self.cancellable.cancel()
    }
}

#[derive(Debug)]
pub struct FireEvent {
    pub player: player::PlayerContainer,
    pub fires: Vec<structs::FireInfo>,

    cancellable: CancellableEvent,
}

impl FireEvent {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CFireEvent);

        use sdk::CFireEvent::*;
        Self {
            player: get_player_from_event(GetSource(event), resource),
            fires: {
                GetFires(event)
                    .iter()
                    .map(|fire| {
                        let pos = sdk::read_fire_info_pos(fire).within_unique_ptr();
                        let pos = read_cpp_vector3(pos);

                        structs::FireInfo {
                            pos,
                            weapon_hash: sdk::read_fire_info_weapon_hash(fire),
                        }
                    })
                    .collect()
            },

            cancellable: CancellableEvent::new(base_event),
        }
    }

    pub fn cancel(&self) -> VoidResult {
        self.cancellable.cancel()
    }
}

#[derive(Debug)]
pub struct ExplosionEvent {
    pub player: player::PlayerContainer,
    pub target: Option<AnyEntity>,
    pub pos: Vector3,
    pub explosion_type: altv_sdk::ExplosionType,
    pub explosion_fx: u32,

    cancellable: CancellableEvent,
}

impl ExplosionEvent {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CExplosionEvent);

        use sdk::CExplosionEvent::*;
        Self {
            player: get_player_from_event(GetSource(event), resource),
            target: get_entity_from_event(GetTarget(event), resource),
            pos: {
                let raw = GetPosition(event).within_unique_ptr();
                read_cpp_vector3(raw)
            },
            explosion_type: {
                let raw = GetExplosionType(event);
                altv_sdk::ExplosionType::from(raw).unwrap()
            },
            explosion_fx: GetExplosionFX(event),

            cancellable: CancellableEvent::new(base_event),
        }
    }

    pub fn cancel(&self) -> VoidResult {
        self.cancellable.cancel()
    }
}

#[derive(Debug)]
pub struct ConnectionQueueAdd {
    info_ptr: *mut sdk::alt::IConnectionInfo,
    info: LazyCell<ConnectionQueueInfo>,
    controller: Rc<ConnectionQueueController>,
}

impl ConnectionQueueAdd {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CConnectionQueueAddEvent);
        let info_ptr = sdk::CConnectionQueueAddEvent::GetConnectionInfo(event);
        let controller = ConnectionQueueController::new(info_ptr);

        resource
            .connection_queue
            .borrow_mut()
            .add(info_ptr, controller.clone());

        Self {
            info_ptr,
            info: LazyCell::new(),
            controller,
        }
    }

    pub fn info(&self) -> &ConnectionQueueInfo {
        if self.info.filled() {
            return self.info.borrow().unwrap();
        }
        self.info
            .fill(unsafe { ConnectionQueueInfo::new(self.info_ptr) })
            .unwrap();
        self.info.borrow().unwrap()
    }

    pub fn controller(&self) -> Rc<ConnectionQueueController> {
        self.controller.clone()
    }
}

#[derive(Debug)]
pub struct ConnectionQueueRemove {
    info_ptr: *mut sdk::alt::IConnectionInfo,
    info: LazyCell<ConnectionQueueInfo>,
}

impl ConnectionQueueRemove {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CConnectionQueueRemoveEvent);
        let info_ptr = sdk::CConnectionQueueRemoveEvent::GetConnectionInfo(event);

        resource.connection_queue.borrow_mut().remove(info_ptr);

        Self {
            info: LazyCell::new(),
            info_ptr,
        }
    }

    pub fn info(&self) -> &ConnectionQueueInfo {
        if self.info.filled() {
            return self.info.borrow().unwrap();
        }
        self.info
            .fill(unsafe { ConnectionQueueInfo::new(self.info_ptr) })
            .unwrap();
        self.info.borrow().unwrap()
    }
}
