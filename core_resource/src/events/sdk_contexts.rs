use std::{cell::Cell, ptr::NonNull, rc::Rc};

use altv_sdk::ffi as sdk;
use autocxx::prelude::*;
use mvalue::ConstMValue;

use crate::{
    alt_resource::AltResource,
    base_objects::{
        connection_info,
        extra_pools::{AnyEntity, AnyWorldObject, ColShapeNonNull},
        player, vehicle, AnyBaseObject,
    },
    events::helpers::{
        base_event_to_specific, get_non_null_base_object_from_event, get_vehicle_from_event,
    },
    helpers::{
        get_entity_by_ptr, get_non_null_entity_by_ptr, get_non_null_player,
        get_non_null_world_object_by_ptr, get_player, read_cpp_str_vec, read_cpp_vector3, Hash,
    },
    resource::Resource,
    vector::Vector3,
    VoidResult,
};

use super::{
    cancellable::CancellableEvent,
    helpers::get_connection_info_from_event,
    // connection_queue::{ConnectionQueueController, ConnectionQueueInfo},
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
            player: get_non_null_player(sdk::CPlayerConnectEvent::GetTarget(event), resource),
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
        let event = base_event_to_specific!(event, CPlayerDisconnectEvent);
        Self {
            player: get_non_null_player(sdk::CPlayerDisconnectEvent::GetTarget(event), resource),
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
pub struct ResourceStart {
    pub resource: Rc<AltResource>,
}

impl ResourceStart {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(event, CResourceStartEvent);

        let resource_ptr = unsafe { sdk::CResourceStartEvent::GetResource(event) };
        let resource_ptr = NonNull::new(resource_ptr).unwrap();
        let resource = resource.alt_resources.borrow_mut().on_start(resource_ptr);

        Self { resource }
    }
}

#[derive(Debug)]
pub struct ResourceStop {
    pub resource: Rc<AltResource>,
}

impl ResourceStop {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(event, CResourceStopEvent);

        let resource_ptr = unsafe { sdk::CResourceStopEvent::GetResource(event) };
        let resource_ptr = NonNull::new(resource_ptr).unwrap();
        let resource = resource.alt_resources.borrow_mut().on_stop(resource_ptr);

        Self { resource }
    }
}

#[derive(Debug)]
pub struct ColshapeEvent {
    pub col_shape: ColShapeNonNull,
    pub world_object: AnyWorldObject,
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
            world_object: get_non_null_world_object_by_ptr(
                sdk::CColShapeEvent::GetEntity(event),
                resource,
            ),
            state,
        }
    }
}

#[derive(Debug)]
pub struct ServerScriptEvent {
    pub name: String,
    pub args: Vec<ConstMValue>,
}

impl ServerScriptEvent {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, _: &Resource) -> Self {
        let event = base_event_to_specific!(event, CServerScriptEvent);
        let name = sdk::CServerScriptEvent::GetName(event).to_string();
        Self {
            name,
            args: sdk::CServerScriptEvent::GetArgs(event)
                .into_iter()
                .map(ConstMValue::from)
                .collect(),
        }
    }
}

#[derive(Debug)]
pub struct ClientScriptEvent {
    pub name: String,
    pub player: player::PlayerContainer,
    pub args: Vec<ConstMValue>,
}

impl ClientScriptEvent {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(event, CClientScriptEvent);
        let name = sdk::CClientScriptEvent::GetName(event).to_string();
        let player = get_non_null_player(sdk::CClientScriptEvent::GetTarget(event), resource);

        Self {
            name,
            player,
            args: sdk::CClientScriptEvent::GetArgs(event)
                .into_iter()
                .map(ConstMValue::from)
                .collect(),
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
        let event = base_event_to_specific!(event, CConsoleCommandEvent);
        Self {
            name: sdk::CConsoleCommandEvent::GetName(event).to_string(),
            args: read_cpp_str_vec(sdk::CConsoleCommandEvent::GetArgs(event)),
        }
    }
}

#[derive(Debug)]
pub struct WeaponDamageEvent {
    pub source: player::PlayerContainer,
    pub source_entity: Option<AnyEntity>,
    pub target: AnyEntity,
    pub weapon_hash: Hash,
    pub body_part: altv_sdk::PlayerBodyPart,
    pub damage: u32,
    pub shot_offset: Vector3,

    event: *mut sdk::alt::CWeaponDamageEvent,
    cancellable: CancellableEvent,
    custom_damage_set: Cell<bool>,
}

impl WeaponDamageEvent {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let weapon_event = base_event_to_specific!(event, CWeaponDamageEvent);

        Self {
            source: get_non_null_player(sdk::CWeaponDamageEvent::GetSource(weapon_event), resource),
            source_entity: get_entity_by_ptr(
                sdk::CWeaponDamageEvent::GetSourceEntity(weapon_event),
                resource,
            ),
            target: get_non_null_entity_by_ptr(
                sdk::CWeaponDamageEvent::GetTarget(weapon_event),
                resource,
            ),
            weapon_hash: sdk::CWeaponDamageEvent::GetWeaponHash(weapon_event),
            body_part: {
                let raw = sdk::CWeaponDamageEvent::GetBodyPart(weapon_event);
                altv_sdk::PlayerBodyPart::try_from(raw).unwrap()
            },
            damage: sdk::CWeaponDamageEvent::GetDamageValue(weapon_event),
            shot_offset: {
                let raw = sdk::CWeaponDamageEvent::GetShotOffset(weapon_event).within_unique_ptr();
                read_cpp_vector3(raw)
            },

            // internal properties
            cancellable: CancellableEvent::new(event),
            event: weapon_event,
            custom_damage_set: Cell::new(false),
        }
    }

    pub fn set_damage(&self, value: u32) -> VoidResult {
        if self.custom_damage_set.get() {
            anyhow::bail!("Damage cannot be set multiple times")
        }
        self.custom_damage_set.set(true);

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
            player: get_non_null_player(sdk::CPlayerDeathEvent::GetTarget(event), resource),
            killer: get_entity_by_ptr(sdk::CPlayerDeathEvent::GetKiller(event), resource),
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
            player: get_non_null_player(sdk::CPlayerDamageEvent::GetTarget(event), resource),
            attacker: get_entity_by_ptr(sdk::CPlayerDamageEvent::GetAttacker(event), resource),
            health_damage: sdk::CPlayerDamageEvent::GetHealthDamage(event),
            armour_damage: sdk::CPlayerDamageEvent::GetArmourDamage(event),
        }
    }
}

macro_rules! player_enter_or_leave_vehicle {
    ($event:ident) => {
        pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
            let event = base_event_to_specific!(event, $event);

            Self {
                player: get_non_null_player(sdk::$event::GetPlayer(event), resource),
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
            player: get_non_null_player(
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
            player: get_non_null_player(sdk::CPlayerWeaponChangeEvent::GetTarget(event), resource),
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
                altv_sdk::PlayerConnectDeniedReason::try_from(raw).unwrap()
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
            player: get_non_null_player(sdk::CPlayerSpawnEvent::GetPlayer(event), resource),
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
            player: get_non_null_player(
                sdk::CPlayerRequestControlEvent::GetPlayer(event),
                resource,
            ),
            entity: get_non_null_entity_by_ptr(
                sdk::CPlayerRequestControlEvent::GetTarget(event),
                resource,
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
            player: get_non_null_player(
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
            player: get_non_null_player(
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
            player: get_non_null_player(GetSource(event), resource),
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
            player: get_non_null_player(GetSource(event), resource),
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
            player: get_non_null_player(GetSource(event), resource),
            target: get_entity_by_ptr(GetTarget(event), resource),
            pos: {
                let raw = GetPosition(event).within_unique_ptr();
                read_cpp_vector3(raw)
            },
            explosion_type: {
                let raw = GetExplosionType(event);
                altv_sdk::ExplosionType::try_from(raw).unwrap()
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
    pub info: connection_info::ConnectionInfoContainer,
}

impl ConnectionQueueAdd {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CConnectionQueueAddEvent);

        Self {
            info: get_connection_info_from_event(
                sdk::CConnectionQueueAddEvent::GetConnectionInfo(event),
                resource,
            ),
        }
    }
}

#[derive(Debug)]
pub struct ConnectionQueueRemove {
    pub info: connection_info::ConnectionInfoContainer,
}

impl ConnectionQueueRemove {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CConnectionQueueRemoveEvent);

        Self {
            info: get_connection_info_from_event(
                sdk::CConnectionQueueRemoveEvent::GetConnectionInfo(event),
                resource,
            ),
        }
    }
}

#[derive(Debug)]
pub struct VehicleAttach {
    pub vehicle: vehicle::VehicleContainer,
    pub attached: vehicle::VehicleContainer,
}

impl VehicleAttach {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CVehicleAttachEvent);

        Self {
            vehicle: get_vehicle_from_event(sdk::CVehicleAttachEvent::GetTarget(event), resource),
            attached: get_vehicle_from_event(
                sdk::CVehicleAttachEvent::GetAttached(event),
                resource,
            ),
        }
    }
}

#[derive(Debug)]
pub struct VehicleDetach {
    pub vehicle: vehicle::VehicleContainer,
    pub detached: vehicle::VehicleContainer,
}

impl VehicleDetach {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CVehicleDetachEvent);

        Self {
            vehicle: get_vehicle_from_event(sdk::CVehicleDetachEvent::GetTarget(event), resource),
            detached: get_vehicle_from_event(
                sdk::CVehicleDetachEvent::GetDetached(event),
                resource,
            ),
        }
    }
}

#[derive(Debug)]
pub struct VehicleDestroy {
    pub vehicle: vehicle::VehicleContainer,
}

impl VehicleDestroy {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CVehicleDestroyEvent);

        Self {
            vehicle: get_vehicle_from_event(sdk::CVehicleDestroyEvent::GetTarget(event), resource),
        }
    }
}

#[derive(Debug)]
pub struct VehicleDamage {
    pub vehicle: vehicle::VehicleContainer,
    pub damager: Option<AnyEntity>,
    pub body_health_damage: u32,
    pub body_additional_health_damage: u32,
    pub engine_health_damage: u32,
    pub petrol_tank_health_damage: u32,
    pub weapon: Hash,
}

impl VehicleDamage {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CVehicleDamageEvent);

        use sdk::CVehicleDamageEvent::*;
        Self {
            vehicle: get_vehicle_from_event(GetTarget(event), resource),
            damager: get_entity_by_ptr(GetDamager(event), resource),
            body_health_damage: GetBodyHealthDamage(event),
            body_additional_health_damage: GetBodyAdditionalHealthDamage(event),
            engine_health_damage: GetEngineHealthDamage(event),
            petrol_tank_health_damage: GetPetrolTankHealthDamage(event),
            weapon: GetDamagedWith(event),
        }
    }
}

#[derive(Debug)]
pub struct VehicleHorn {
    pub vehicle: vehicle::VehicleContainer,
    pub player: player::PlayerContainer,
    pub state: bool,
}

impl VehicleHorn {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CVehicleHornEvent);

        use sdk::CVehicleHornEvent::*;
        Self {
            vehicle: get_vehicle_from_event(GetTarget(event), resource),
            player: get_non_null_player(GetReporter(event), resource),
            state: GetToggle(event),
        }
    }
}

#[derive(Debug)]
pub struct VehicleSiren {
    pub vehicle: vehicle::VehicleContainer,
    pub state: bool,
}

impl VehicleSiren {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CVehicleSirenEvent);

        Self {
            vehicle: get_vehicle_from_event(sdk::CVehicleSirenEvent::GetTarget(event), resource),
            state: sdk::CVehicleSirenEvent::GetToggle(event),
        }
    }
}

#[derive(Debug)]
pub struct NetownerChange {
    pub entity: AnyEntity,
    pub new_net_owner: Option<player::PlayerContainer>,
    pub old_net_owner: Option<player::PlayerContainer>,
}

impl NetownerChange {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CNetOwnerChangeEvent);

        use sdk::CNetOwnerChangeEvent::*;
        Self {
            entity: get_non_null_entity_by_ptr(GetTarget(event), resource),
            new_net_owner: get_player(GetNewOwner(event), resource),
            old_net_owner: get_player(GetOldOwner(event), resource),
        }
    }
}

#[derive(Debug)]
pub struct GlobalMetaChange {
    pub key: String,
    pub new_value: ConstMValue,
    pub old_value: ConstMValue,
}

impl GlobalMetaChange {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, _: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CGlobalMetaDataChangeEvent);

        use sdk::CGlobalMetaDataChangeEvent::*;
        Self {
            key: GetKey(event).to_string(),
            new_value: ConstMValue::new(GetVal(event).within_unique_ptr()),
            old_value: ConstMValue::new(GetOldVal(event).within_unique_ptr()),
        }
    }
}

#[derive(Debug)]
pub struct GlobalSyncedMetaChange {
    pub key: String,
    pub new_value: ConstMValue,
    pub old_value: ConstMValue,
}

impl GlobalSyncedMetaChange {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, _: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CGlobalSyncedMetaDataChangeEvent);

        use sdk::CGlobalSyncedMetaDataChangeEvent::*;
        Self {
            key: GetKey(event).to_string(),
            new_value: ConstMValue::new(GetVal(event).within_unique_ptr()),
            old_value: ConstMValue::new(GetOldVal(event).within_unique_ptr()),
        }
    }
}

#[derive(Debug)]
pub struct SyncedMetaChange {
    pub key: String,
    pub base_object: AnyBaseObject,
    pub new_value: ConstMValue,
    pub old_value: ConstMValue,
}

impl SyncedMetaChange {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CSyncedMetaDataChangeEvent);

        use sdk::CSyncedMetaDataChangeEvent::*;
        Self {
            key: GetKey(event).to_string(),
            base_object: get_non_null_base_object_from_event(GetTarget(event), resource),
            new_value: ConstMValue::new(GetVal(event).within_unique_ptr()),
            old_value: ConstMValue::new(GetOldVal(event).within_unique_ptr()),
        }
    }
}

#[derive(Debug)]
pub struct StreamSyncedMetaChange {
    pub key: String,
    pub base_object: AnyBaseObject,
    pub new_value: ConstMValue,
    pub old_value: ConstMValue,
}

impl StreamSyncedMetaChange {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CStreamSyncedMetaDataChangeEvent);

        use sdk::CStreamSyncedMetaDataChangeEvent::*;
        Self {
            key: GetKey(event).to_string(),
            base_object: get_non_null_base_object_from_event(GetTarget(event), resource),
            new_value: ConstMValue::new(GetVal(event).within_unique_ptr()),
            old_value: ConstMValue::new(GetOldVal(event).within_unique_ptr()),
        }
    }
}

#[derive(Debug)]
pub struct LocalSyncedMetaChange {
    pub key: String,
    pub player: player::PlayerContainer,
    pub new_value: ConstMValue,
    pub old_value: ConstMValue,
}

impl LocalSyncedMetaChange {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CLocalMetaDataChangeEvent);

        use sdk::CLocalMetaDataChangeEvent::*;
        Self {
            key: GetKey(event).to_string(),
            player: get_non_null_player(GetTarget(event), resource),
            new_value: ConstMValue::new(GetVal(event).within_unique_ptr()),
            old_value: ConstMValue::new(GetOldVal(event).within_unique_ptr()),
        }
    }
}

#[derive(Debug)]
pub struct VoiceConnectionEvent {
    pub state: altv_sdk::VoiceConnectionState,
}

impl VoiceConnectionEvent {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, _: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CVoiceConnectionEvent);
        Self {
            state: altv_sdk::VoiceConnectionState::try_from(sdk::CVoiceConnectionEvent::GetState(
                event,
            ))
            .unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct RequestSyncedScene {
    pub source: player::PlayerContainer,
    pub scene_id: i32,

    cancellable: CancellableEvent,
}

impl RequestSyncedScene {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CRequestSyncedSceneEvent);
        Self {
            source: get_non_null_player(sdk::CRequestSyncedSceneEvent::GetSource(event), resource),
            scene_id: sdk::CRequestSyncedSceneEvent::GetSceneID(event),
            cancellable: CancellableEvent::new(base_event),
        }
    }

    pub fn cancel(&self) -> VoidResult {
        self.cancellable.cancel()
    }
}

#[derive(Debug)]
pub struct StartSyncedScene {
    pub source: player::PlayerContainer,
    pub scene_id: i32,
    pub start_pos: Vector3,
    pub start_rot: Vector3,
    pub anim_dict_hash: Hash,
    pub entity_and_anim_hash_pairs: Vec<(AnyEntity, Hash)>,

    cancellable: CancellableEvent,
}

impl StartSyncedScene {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CStartSyncedSceneEvent);

        use sdk::CStartSyncedSceneEvent::*;
        Self {
            source: get_non_null_player(GetSource(event), resource),
            scene_id: GetSceneID(event),
            start_pos: {
                let raw = GetStartPosition(event).within_unique_ptr();
                read_cpp_vector3(raw)
            },
            start_rot: {
                let raw = GetStartRotation(event).within_unique_ptr();
                read_cpp_vector3(raw)
            },
            anim_dict_hash: GetAnimDictHash(event),
            entity_and_anim_hash_pairs: {
                let raw = GetEntityAndAnimHashPairs(event).within_unique_ptr();
                let raw = sdk::read_entity_anim_hash_pairs(raw.as_ref().unwrap());
                let mut pairs = Vec::new();
                for e in raw.into_iter() {
                    let entity = get_non_null_entity_by_ptr(
                        sdk::read_entity_anim_hash_pair_entity(e),
                        resource,
                    );
                    let anim_hash = unsafe { sdk::read_entity_anim_hash_pair_anim_hash(e) };
                    pairs.push((entity, anim_hash));
                }
                pairs
            },

            cancellable: CancellableEvent::new(base_event),
        }
    }

    pub fn cancel(&self) -> VoidResult {
        self.cancellable.cancel()
    }
}

#[derive(Debug)]
pub struct StopSyncedScene {
    pub source: player::PlayerContainer,
    pub scene_id: i32,

    cancellable: CancellableEvent,
}

impl StopSyncedScene {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CStopSyncedSceneEvent);
        Self {
            source: get_non_null_player(sdk::CStopSyncedSceneEvent::GetSource(event), resource),
            scene_id: sdk::CStopSyncedSceneEvent::GetSceneID(event),

            cancellable: CancellableEvent::new(base_event),
        }
    }

    pub fn cancel(&self) -> VoidResult {
        self.cancellable.cancel()
    }
}

#[derive(Debug)]
pub struct UpdateSyncedScene {
    pub source: player::PlayerContainer,
    pub scene_id: i32,
    pub start_rate: f32,

    cancellable: CancellableEvent,
}

impl UpdateSyncedScene {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CUpdateSyncedSceneEvent);
        Self {
            source: get_non_null_player(sdk::CUpdateSyncedSceneEvent::GetSource(event), resource),
            scene_id: sdk::CUpdateSyncedSceneEvent::GetSceneID(event),
            start_rate: sdk::CUpdateSyncedSceneEvent::GetStartRate(event),

            cancellable: CancellableEvent::new(base_event),
        }
    }

    pub fn cancel(&self) -> VoidResult {
        self.cancellable.cancel()
    }
}

#[derive(Debug)]
pub struct ClientDeleteObjectEvent {
    pub source: player::PlayerContainer,

    cancellable: CancellableEvent,
}

impl ClientDeleteObjectEvent {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CClientDeleteObjectEvent);
        Self {
            source: get_non_null_player(sdk::CClientDeleteObjectEvent::GetTarget(event), resource),

            cancellable: CancellableEvent::new(base_event),
        }
    }

    pub fn cancel(&self) -> VoidResult {
        self.cancellable.cancel()
    }
}

#[derive(Debug)]
pub struct ClientRequestObjectEvent {
    pub source: player::PlayerContainer,
    pub model: Hash,
    pub pos: Vector3,

    cancellable: CancellableEvent,
}

impl ClientRequestObjectEvent {
    pub(crate) unsafe fn new(base_event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(base_event, CClientRequestObjectEvent);
        Self {
            source: get_non_null_player(sdk::CClientRequestObjectEvent::GetTarget(event), resource),
            model: sdk::CClientRequestObjectEvent::GetModel(event),
            pos: {
                let raw = sdk::CClientRequestObjectEvent::GetPosition(event).within_unique_ptr();
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
pub struct PlayerHeal {
    pub player: player::PlayerContainer,
    pub old_health: u16,
    pub new_health: u16,
    pub old_armour: u16,
    pub new_armour: u16,
}

impl PlayerHeal {
    pub(crate) unsafe fn new(event: altv_sdk::CEventPtr, resource: &Resource) -> Self {
        let event = base_event_to_specific!(event, CPlayerHealEvent);

        use sdk::CPlayerHealEvent as E;
        Self {
            player: get_non_null_player(E::GetTarget(event), resource),
            old_health: E::GetOldHealth(event),
            new_health: E::GetNewHealth(event),
            old_armour: E::GetOldArmour(event),
            new_armour: E::GetNewArmour(event),
        }
    }
}
