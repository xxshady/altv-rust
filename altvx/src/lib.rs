use core_altvx::{exports, VoidResult};

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
    EntityId,
    Player,
    PlayerContainer,
    Vector2,
    Vector3,
    Vehicle,
    VirtualEntity,
    VirtualEntityGroup,
    AnyEntity,
    AttachToEntityBoneIndex,
    AttachToEntityBoneName,
    PlayerDateTime,
    PlayerHeadBlendData,
    anyhow,
    hash,
    Hash,
    VehicleModelInfo,
};

pub use altv_sdk::{
    ColShapeType, ExplosionType, PlayerBodyPart, PlayerConnectDeniedReason, VehicleModelType,
};

pub mod events;
pub mod mvalue;

pub mod prelude {
    pub use super::exports::{
        BaseObjectMeta, Entity, LocalMeta, StreamSyncedMeta, SyncedMeta, ValidBaseObject,
        WorldObject,
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
    pub use super::exports::{init as core_init, ModuleHandlers, ResourceHandlers};
    pub use altv_sdk::ffi::{alt::ICore, set_alt_core};

    pub fn init(
        full_main_path: String,
        resource_state: &mut ResourceHandlers,
        module_handlers: ModuleHandlers,
    ) {
        core_init(full_main_path, resource_state, module_handlers);
    }
}
