//! Server-side Rust API for alt:V
//!
//! [How to use?](https://github.com/xxshady/altv-rust#how-to-use)

use core_resource::exports;

#[rustfmt::skip]
pub use exports::{
    logging::{
        log_macro as log,
        log_warn_macro as log_warn,
        log_error_macro as log_error,
        log,
        log_warn,
        log_error,
    },
    BaseObjectWrapper,
    SyncId,
    Vector2,
    Vector3,

    // base objects
    ColShape, ColShapeContainer,
    Player, PlayerContainer,
    Vehicle, VehicleContainer,
    Ped, PedContainer,
    NetworkObject, NetworkObjectContainer,
    VirtualEntity, VirtualEntityContainer,
    VirtualEntityGroup, VirtualEntityGroupContainer,
    Blip, BlipContainer,
    VoiceChannel, VoiceChannelContainer,
    Marker, MarkerContainer,
    Checkpoint, CheckpointContainer,

    WorldObject,
    Entity,
    AnyEntity,
    AttachToEntityBoneIndex,
    AttachToEntityBoneName,
    PlayerDateTime,
    PlayerHeadBlendData,
    PlayAnimation,
    hash,
    Hash,
    VehicleModelInfo,
    PedModelInfo,
    WeaponModelInfo,
    Rgba,
    Quaternion,
    core_funcs::*,
    config_node::*,
    Resource,
    ColShapy,
    result::{VoidResult, IntoVoidResult, SomeResult},
    AnyBaseObject,
    BaseObjectPoolFuncs,
    base_object,
    BaseObjectId,
    Timer,
    AnimationFlags,
    AmmoType,
    AmmoFlags,

    anyhow,
    serde,
    erased_serde,
    serde_bytes::{
        self,
        ByteBuf,
        // Bytes TODO: implement bytes deserialization
    },
};

pub use altv_sdk::{
    BaseObjectType, BlipType, ColShapeType, ExplosionType, MarkerType, PlayerBodyPart,
    PlayerConnectDeniedReason, VehicleModelType, AmmoSpecialType,
};

pub const DEFAULT_DIMENSION: i32 = 0;
pub const GLOBAL_DIMENSION: i32 = i32::MIN;
/// # Examples
///
/// ```rust
/// # fn test() -> altv::VoidResult {
/// altv::events::on_player_enter_vehicle(|event| {
///     if event.seat == altv::DRIVER_SEAT {
///         altv::log!("Player: {} entered vehicle as driver", event.player.name()?);
///     }
///     Ok(())
/// });
/// # Ok(()) }
/// ```
pub const DRIVER_SEAT: u8 = 1;

pub mod events;
pub mod meta;
pub mod mvalue;

#[rustfmt::skip]
pub mod prelude {
    pub use super::exports::{
        config_node::Config,
        meta::{
            MetaEntry,
            BaseObjectMetaEntry,
            NormalBaseObjectMeta,
            SyncedBaseObjectMeta,
            StreamSyncedEntityMeta,
            StreamSyncedCheckpointMeta, 
            StreamSyncedVirtualEntityMeta,
            LocalPlayerMeta, 
        },
        mvalue::DeserializeMValueArgs,

        BaseObjectPoolFuncs,
        ColShapy,
        Entity,
        ValidBaseObject,
        WorldObject,
    };
}

/// # Examples
///
/// ```rust
/// # fn test() -> altv::VoidResult {
/// altv::set_timeout(
///     move || {
///         altv::log!("this message will be printed once, after 1.5s");
///     },
///     1500,
/// );
/// # Ok(()) }
/// ```
///
/// With error handling
/// ```rust
/// # fn test() -> altv::VoidResult {
/// fn something_that_can_fail() -> altv::VoidResult {
///     Ok(())
/// }
///
/// altv::set_timeout(
///     move || {
///         altv::log!("this message will be printed once, after 1.5s");
///         something_that_can_fail()?;
///         Ok(())
///     },
///     1500,
/// );
/// # Ok(()) }
/// ```
///
/// Destroying (clearing) timer
/// ```rust
/// # fn test() -> altv::VoidResult {
/// let mut timer = altv::set_timeout(
///     move || {
///         altv::log!("this message will never be printed");
///     },
///     1500,
/// );
///
/// // Returns error if timer was already destroyed
/// timer.destroy()?;
/// # Ok(()) }
/// ```
pub fn set_timeout<V: IntoVoidResult>(
    mut callback: impl FnMut() -> V + 'static,
    millis: u64,
) -> Timer {
    exports::create_timer(
        Box::new(move || callback().into_void_result()),
        millis,
        true,
    )
}

/// # Examples
///
/// Without error handling
/// ```rust
/// # fn test() -> altv::VoidResult {
/// altv::set_interval(
///     move || {
///         altv::log!("this message will be printed every 1.5s");
///     },
///     1500,
/// );
/// # Ok(()) }
/// ```
///
/// With error handling
/// ```rust
/// # fn test() -> altv::VoidResult {
/// fn something_that_can_fail() -> altv::VoidResult {
///     Ok(())
/// }
///
/// altv::set_interval(
///     move || {
///         altv::log!("this message will be printed every 1.5s");
///         something_that_can_fail()?;
///         Ok(())
///     },
///     1500,
/// );
/// # Ok(()) }
/// ```
///
/// Destroying (clearing) timer
/// ```rust
/// # fn test() -> altv::VoidResult {
/// let mut timer = altv::set_interval(
///     move || {
///         altv::log!("this message will never be printed");
///     },
///     1500,
/// );
///
/// // Returns error if timer was already destroyed
/// timer.destroy()?;
/// # Ok(()) }
/// ```
pub fn set_interval<V: IntoVoidResult>(
    mut callback: impl FnMut() -> V + 'static,
    millis: u64,
) -> Timer {
    exports::create_timer(
        Box::new(move || callback().into_void_result()),
        millis,
        false,
    )
}

pub use resource_main_macro::resource_main_func as main;
// __internal is intended for resource_main_func proc macro ^
#[doc(hidden)]
pub mod __internal {
    pub use super::exports::{init as core_init, ModuleHandlers, ResourceHandlers, ResourceName};
    pub use altv_sdk::ffi::{alt::ICore, set_alt_core};

    pub fn init(
        name: ResourceName,
        resource_state: &mut ResourceHandlers,
        module_handlers: ModuleHandlers,
    ) {
        core_init(name, resource_state, module_handlers);
    }
}
