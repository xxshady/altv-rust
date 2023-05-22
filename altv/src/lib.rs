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
    AnimationFlags,
    hash,
    Hash,
    VehicleModelInfo,
    PedModelInfo,
    RGBA,
    Quaternion,
    core_funcs::*,
    config_node::*,
    Resource,
    ColShapy,
    result::{VoidResult, IntoVoidResult, SomeResult},
    AnyBaseObject,
    BaseObjectPoolFuncs,
    base_object,
};

pub use altv_sdk::{
    BaseObjectType, BlipType, ColShapeType, ExplosionType, MarkerType, PlayerBodyPart,
    PlayerConnectDeniedReason, VehicleModelType,
};

pub use anyhow;
pub use erased_serde;
pub use serde_bytes::{
    self,
    ByteBuf,
    // Bytes TODO: implement bytes deserialization
};

pub const DEFAULT_DIMENSION: i32 = 0;
pub const GLOBAL_DIMENSION: i32 = i32::MIN;

pub mod events;
// pub mod meta;
// pub mod mvalue;

// TEST
// TODO: add them as direct deps of altv crate
pub use exports::__mvalue::{self, DynMValue, DynMValueArgs};
pub use exports::__serde;

pub mod prelude {
    pub use super::exports::{
        config_node::Config,
        // meta::{
        //     BaseObjectMetaEntry, LocalPlayerMeta, MetaEntry, NormalBaseObjectMeta,
        //     StreamSyncedCheckpointMeta, StreamSyncedEntityMeta, StreamSyncedVirtualEntityMeta,
        //     SyncedBaseObjectMeta,
        // },
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
/// altv::set_timeout(
///     move || {
///         altv::log!("this message will be printed once, after 1.5s");
///     },
///     1500,
/// );
/// ```
///
/// With error handling
/// ```rust
/// altv::set_timeout(
///     move || {
///         altv::log!("this message will be printed once, after 1.5s");
///         something_that_can_fail()?;
///         Ok(())
///     },
///     1500,
/// );
/// ```
pub fn set_timeout<V: IntoVoidResult>(mut callback: impl FnMut() -> V + 'static, millis: u64) {
    exports::create_timer(
        Box::new(move || callback().into_void_result()),
        millis,
        true,
    );
}

/// # Examples
///
/// Without error handling
/// ```rust
/// altv::set_interval(
///     move || {
///         altv::log!("this message will be printed every 1.5s");
///     },
///     1500,
/// );
/// ```
///
/// With error handling
/// ```rust
/// altv::set_interval(
///     move || {
///         altv::log!("this message will be printed every 1.5s");
///         something_that_can_fail()?;
///         Ok(())
///     },
///     1500,
/// );
/// ```
pub fn set_interval<V: IntoVoidResult>(mut callback: impl FnMut() -> V + 'static, millis: u64) {
    exports::create_timer(
        Box::new(move || callback().into_void_result()),
        millis,
        false,
    );
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
