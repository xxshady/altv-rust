#![allow(non_camel_case_types)]
use primitive_enum::primitive_enum;
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
	OBJECT
,
}