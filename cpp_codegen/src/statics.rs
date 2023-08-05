use std::collections::HashMap;

lazy_static::lazy_static! {
    pub static ref SUPPORTED_CPP_TYPES: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("void", "void"),
            ("bool", "bool"),

            ("uint8_t", "u8"),
            ("uint16_t", "u16"),
            ("uint32_t", "u32"),
            ("uint64_t", "u64"),

            ("int8_t", "i8"),
            ("int16_t", "i16"),
            ("int", "cpp_int"), // why? for some reason sdk uses int and int32_t at the same time
            ("int32_t", "i32"),
            ("int64_t", "i64"),

            ("float", "f32"),
            ("double", "f64"),

            ("std::string", "std::string"),
            ("std::string&", "StdStringClone"),
            ("MValue", "MValueMutWrapper"),
            ("MValueConst", "ConstMValueWrapper"),
            ("MValueArgs&", "MValueWrapperVec"),

            ("IBaseObject*", "alt::IBaseObject*"),
            ("IVehicle*", "alt::IVehicle*"),
            ("IPed*", "alt::IPed*"),
            ("IObject*", "alt::IObject*"),
            ("IEntity*", "alt::IEntity*"),
            ("IPlayer*", "alt::IPlayer*"),
            ("IColShape*", "alt::IColShape*"),
            ("IResource*", "alt::IResource*"),
            ("ICore*", "alt::ICore*"),
            ("IVirtualEntityGroup*", "alt::IVirtualEntityGroup*"),
            ("IVirtualEntity*", "alt::IVirtualEntity*"),
            ("IConnectionInfo*", "alt::IConnectionInfo*"),
            ("VehicleModelInfo&", "alt::VehicleModelInfo*"),
            ("PedModelInfo&", "alt::PedModelInfo*"),
            ("WeaponModelInfo&", "alt::WeaponModelInfo*"),
            ("IWorldObject*", "alt::IWorldObject*"),
            ("IBlip*", "alt::IBlip*"),
            ("IVoiceChannel*", "alt::IVoiceChannel*"),
            ("IMarker*", "alt::IMarker*"),
            ("ICheckpoint*", "alt::ICheckpoint*"),

            ("alt::Prop", "alt::Prop"),
            ("alt::DlcProp", "alt::DlcProp"),
            ("alt::Cloth", "alt::Cloth"),
            ("alt::DlcCloth", "alt::DlcCloth"),
            ("HeadOverlay", "alt::HeadOverlay"),
            ("HeadBlendData", "alt::HeadBlendData"),
            ("alt::CEvent::Type", "EventType"),
            ("IBlip::BlipType", "BlipType"),
            ("IMarker::MarkerType", "MarkerType"),
            ("AmmoSpecialType", "AmmoSpecialType_t"),
            ("AmmoFlags", "alt::AmmoFlags"),

            ("alt::Position", "Vector3Wrapper"),
            ("Position", "Vector3Wrapper"),
            ("Vector3f", "Vector3Wrapper"),
            ("Vector2f", "Vector2Wrapper"),
            ("RGBA", "RGBAWrapper"),
            ("alt::RGBA", "RGBAWrapper"),
            ("std::vector<uint32_t>", "std::vector<u32>"),
            ("std::vector<std::string>", "std::vector<std::string>"),
            ("std::vector<Weapon>", "std::vector<WeaponWrapper>"),
            ("std::vector<std::string>&", "std::vector<std::string>"),
            ("std::vector<Vector2f>", "Vector2Vec"),
            ("std::vector<FireInfo>&", "std::vector<FireInfoWrapper>"),
            ("Quaternion", "alt::Quaternion"),
            ("std::vector<IBaseObject*>", "BaseObjectVector"),
            ("std::vector<IResource*>", "ResourceVector"),
            ("std::vector<IPlayer*>", "PlayerVector"),
            ("std::vector<std::pair<IEntity*, int32_t>>", "std::vector<StreamedEntityWrapper>"),
            ("std::vector<CDecoration>", "std::vector<alt::CDecoration>"),

            ("Rotation", "Vector3Wrapper"),
            ("bool*", "bool*"),

            ("Config::Value::ValuePtr", "Config::Value::ValuePtr"),

            ("std::unordered_map<std::string, MValue>", "MValueUnorderedMapWrapper"),
            ("std::unordered_map<std::shared_ptr<IEntity>, uint32_t>", "EntityAnimHashPairsWrapper"),
            ("CVoiceConnectionEvent::State", "VoiceConnectionState"),
        ])
    };

    pub static ref SUPPORTED_CPP_TYPES_IN_CLASSES: HashMap<&'static str, &'static str> = {
        HashMap::from([
            ("IBaseObject::Type", "BaseObjectType"),
            ("IColShape::ColShapeType", "ColShapeType"),
            ("IBlip::BlipType", "BlipType"),
            ("IMarker::MarkerType", "MarkerType"),
            ("CWeaponDamageEvent::BodyPart", "WeaponDamageEventBodyPart"),
            ("CEvent::Type", "EventType"),
            ("CPlayerConnectDeniedEvent::Reason", "PlayerConnectDeniedReason"),
            ("CExplosionEvent::ExplosionType", "ExplosionType"),
            ("CVoiceConnectionEvent::State", "VoiceConnectionState"),
        ])
    };
}

type CustomCaller = fn(String) -> String;

pub static HEADERS_CUSTOM: &[(&str, &str, CustomCaller)] =
    &[("ICore", "../altv_sdk/cpp-sdk/ICore.h", |v| {
        format!("alt::ICore::Instance().{v}")
    })];

pub static HEADERS: &[(&str, &str)] = &[
    // objects
    ("IBaseObject", "../altv_sdk/cpp-sdk/objects/IBaseObject.h"),
    ("IWorldObject", "../altv_sdk/cpp-sdk/objects/IWorldObject.h"),
    ("IEntity", "../altv_sdk/cpp-sdk/objects/IEntity.h"),
    ("IPlayer", "../altv_sdk/cpp-sdk/objects/IPlayer.h"),
    ("IVehicle", "../altv_sdk/cpp-sdk/objects/IVehicle.h"),
    ("IPed", "../altv_sdk/cpp-sdk/objects/IPed.h"),
    ("IObject", "../altv_sdk/cpp-sdk/objects/IObject.h"),
    (
        "IColShape",
        "../altv_sdk/cpp-sdk/script-objects/IColShape.h",
    ),
    ("IBlip", "../altv_sdk/cpp-sdk/script-objects/IBlip.h"),
    (
        "ICheckpoint",
        "../altv_sdk/cpp-sdk/script-objects/ICheckpoint.h",
    ),
    (
        "IVirtualEntityGroup",
        "../altv_sdk/cpp-sdk/script-objects/IVirtualEntityGroup.h",
    ),
    (
        "IVirtualEntity",
        "../altv_sdk/cpp-sdk/script-objects/IVirtualEntity.h",
    ),
    (
        "IVoiceChannel",
        "../altv_sdk/cpp-sdk/script-objects/IVoiceChannel.h",
    ),
    ("IMarker", "../altv_sdk/cpp-sdk/script-objects/IMarker.h"),
    (
        "ICheckpoint",
        "../altv_sdk/cpp-sdk/script-objects/ICheckpoint.h",
    ),
    (
        "IConnectionInfo",
        "../altv_sdk/cpp-sdk/script-objects/IConnectionInfo.h",
    ),
    (
        "VehicleModelInfo",
        "../altv_sdk/cpp-sdk/types/VehicleModelInfo.h",
    ),
    ("PedModelInfo", "../altv_sdk/cpp-sdk/types/PedModelInfo.h"),
    (
        "WeaponModelInfo",
        "../altv_sdk/cpp-sdk/types/WeaponModelInfo.h",
    ),
    // events
    ("CEvent", "../altv_sdk/cpp-sdk/events/CEvent.h"),
    (
        "CWeaponDamageEvent",
        "../altv_sdk/cpp-sdk/events/CWeaponDamageEvent.h",
    ),
    (
        "CColShapeEvent",
        "../altv_sdk/cpp-sdk/events/CColShapeEvent.h",
    ),
    (
        "CConsoleCommandEvent",
        "../altv_sdk/cpp-sdk/events/CConsoleCommandEvent.h",
    ),
    (
        "CClientScriptEvent",
        "../altv_sdk/cpp-sdk/events/CClientScriptEvent.h",
    ),
    (
        "CServerScriptEvent",
        "../altv_sdk/cpp-sdk/events/CServerScriptEvent.h",
    ),
    (
        "CConnectionQueueAddEvent",
        "../altv_sdk/cpp-sdk/events/CConnectionQueueAddEvent.h",
    ),
    (
        "CConnectionQueueRemoveEvent",
        "../altv_sdk/cpp-sdk/events/CConnectionQueueRemoveEvent.h",
    ),
    // entity
    (
        "CNetOwnerChangeEvent",
        "../altv_sdk/cpp-sdk/events/CNetOwnerChangeEvent.h",
    ),
    // player
    (
        "CPlayerConnectEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerConnectEvent.h",
    ),
    (
        "CPlayerDisconnectEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerDisconnectEvent.h",
    ),
    (
        "CPlayerDeathEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerDeathEvent.h",
    ),
    (
        "CPlayerDamageEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerDamageEvent.h",
    ),
    (
        "CPlayerEnteringVehicleEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerEnteringVehicleEvent.h",
    ),
    (
        "CPlayerEnterVehicleEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerEnterVehicleEvent.h",
    ),
    (
        "CPlayerLeaveVehicleEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerLeaveVehicleEvent.h",
    ),
    (
        "CPlayerChangeAnimationEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerChangeAnimationEvent.h",
    ),
    (
        "CPlayerChangeVehicleSeatEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerChangeVehicleSeatEvent.h",
    ),
    (
        "CPlayerWeaponChangeEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerWeaponChangeEvent.h",
    ),
    (
        "CPlayerConnectDeniedEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerConnectDeniedEvent.h",
    ),
    (
        "CPlayerSpawnEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerSpawnEvent.h",
    ),
    (
        "CPlayerRequestControlEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerRequestControlEvent.h",
    ),
    (
        "CPlayerDimensionChangeEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerDimensionChangeEvent.h",
    ),
    (
        "CPlayerChangeInteriorEvent",
        "../altv_sdk/cpp-sdk/events/CPlayerChangeInteriorEvent.h",
    ),
    // vehicle
    (
        "CVehicleAttachEvent",
        "../altv_sdk/cpp-sdk/events/CVehicleAttachEvent.h",
    ),
    (
        "CVehicleDamageEvent",
        "../altv_sdk/cpp-sdk/events/CVehicleDamageEvent.h",
    ),
    (
        "CVehicleDestroyEvent",
        "../altv_sdk/cpp-sdk/events/CVehicleDestroyEvent.h",
    ),
    (
        "CVehicleDetachEvent",
        "../altv_sdk/cpp-sdk/events/CVehicleDetachEvent.h",
    ),
    (
        "CVehicleHornEvent",
        "../altv_sdk/cpp-sdk/events/CVehicleHornEvent.h",
    ),
    (
        "CVehicleSirenEvent",
        "../altv_sdk/cpp-sdk/events/CVehicleSirenEvent.h",
    ),
    (
        "CStartProjectileEvent",
        "../altv_sdk/cpp-sdk/events/CStartProjectileEvent.h",
    ),
    ("CFireEvent", "../altv_sdk/cpp-sdk/events/CFireEvent.h"),
    (
        "CExplosionEvent",
        "../altv_sdk/cpp-sdk/events/CExplosionEvent.h",
    ),
    (
        "CExplosionEvent",
        "../altv_sdk/cpp-sdk/events/CExplosionEvent.h",
    ),
    // meta
    (
        "CMetaChangeEvent",
        "../altv_sdk/cpp-sdk/events/CMetaDataChangeEvent.h",
    ),
    (
        "CGlobalMetaDataChangeEvent",
        "../altv_sdk/cpp-sdk/events/CGlobalMetaDataChangeEvent.h",
    ),
    (
        "CGlobalSyncedMetaDataChangeEvent",
        "../altv_sdk/cpp-sdk/events/CGlobalSyncedMetaDataChangeEvent.h",
    ),
    (
        "CSyncedMetaDataChangeEvent",
        "../altv_sdk/cpp-sdk/events/CSyncedMetaDataChangeEvent.h",
    ),
    (
        "CStreamSyncedMetaDataChangeEvent",
        "../altv_sdk/cpp-sdk/events/CStreamSyncedMetaDataChangeEvent.h",
    ),
    (
        "CLocalMetaDataChangeEvent",
        "../altv_sdk/cpp-sdk/events/CLocalMetaDataChangeEvent.h",
    ),
    // resource
    (
        "CResourceStopEvent",
        "../altv_sdk/cpp-sdk/events/CResourceStopEvent.h",
    ),
    (
        "CResourceStartEvent",
        "../altv_sdk/cpp-sdk/events/CResourceStartEvent.h",
    ),
    // voice
    (
        "CVoiceConnectionEvent",
        "../altv_sdk/cpp-sdk/events/CVoiceConnectionEvent.h",
    ),
    // synced scene
    (
        "CRequestSyncedSceneEvent",
        "../altv_sdk/cpp-sdk/events/CRequestSyncedSceneEvent.h",
    ),
    (
        "CStartSyncedSceneEvent",
        "../altv_sdk/cpp-sdk/events/CStartSyncedSceneEvent.h",
    ),
    (
        "CStopSyncedSceneEvent",
        "../altv_sdk/cpp-sdk/events/CStopSyncedSceneEvent.h",
    ),
    (
        "CUpdateSyncedSceneEvent",
        "../altv_sdk/cpp-sdk/events/CUpdateSyncedSceneEvent.h",
    ),
    ("IResource", "../altv_sdk/cpp-sdk/IResource.h"),
];
