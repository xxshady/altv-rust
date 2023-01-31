use primitive_enum::primitive_enum;

// TODO: copy IBaseObject::Type enum from IBaseObject.h automatically
primitive_enum! { BaseObjectType u8 ;
    PLAYER,
    VEHICLE,
    BLIP,
    WEBVIEW,
    VOICE_CHANNEL,
    COLSHAPE,
    CHECKPOINT,
    WEBSOCKET_CLIENT,
    HTTP_CLIENT,
    AUDIO,
    RML_ELEMENT,
    RML_DOCUMENT,
    LOCAL_PLAYER,
    OBJECT,
}
