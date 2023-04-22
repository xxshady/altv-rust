//! Server-side Rust API for alt:V
//!
//! [How to use?](https://github.com/xxshady/altv-rust#how-to-use)

use core_resource::{exports, VoidResult};

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

    ColShape,
    SyncId,
    Player,
    PlayerContainer,
    Vector2,
    Vector3,
    Vehicle,
    VirtualEntity,
    VirtualEntityGroup,
    Blip,
    VoiceChannel,
    AnyEntity,
    AttachToEntityBoneIndex,
    AttachToEntityBoneName,
    PlayerDateTime,
    PlayerHeadBlendData,
    PlayAnimation,
    AnimationFlags,
    anyhow,
    hash,
    Hash,
    VehicleModelInfo,
    PedModelInfo,
    RGBA,
    Quaternion,
    core_funcs::*,
    config_node::*,
    Resource,
};

pub use altv_sdk::{
    BaseObjectType, ColShapeType, ExplosionType, PlayerBodyPart, PlayerConnectDeniedReason,
    VehicleModelType,
};

pub mod events;
pub mod mvalue;

pub mod prelude {
    pub use super::exports::{
        config_node::Config, BaseObjectMeta, Entity, LocalMeta, StreamSyncedMeta, SyncedMeta,
        ValidBaseObject, WorldObject,
    };
}

pub fn set_timeout(callback: impl FnMut() -> VoidResult + 'static, millis: u64) {
    exports::create_timer(Box::new(callback), millis, true);
}

pub fn set_interval(callback: impl FnMut() -> VoidResult + 'static, millis: u64) {
    exports::create_timer(Box::new(callback), millis, false);
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
