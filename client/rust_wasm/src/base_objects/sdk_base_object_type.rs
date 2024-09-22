use serde_repr::{Deserialize_repr, Serialize_repr};

// TODO: generate it from sdk or TS typings
#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SdkBaseObjectType {
  PLAYER,           // GAME_ENTITY_CREATE -> GAME_ENTITY_DESTROY
  VEHICLE,          // GAME_ENTITY_CREATE -> GAME_ENTITY_DESTROY
  PED,              // GAME_ENTITY_CREATE -> GAME_ENTITY_DESTROY
  OBJECT,           // GAME_ENTITY_CREATE -> GAME_ENTITY_DESTROY
  BLIP,             // WORLD_OBJECT_STREAM_IN -> WORLD_OBJECT_STREAM_OUT
  WEBVIEW,          // local
  VOICE_CHANNEL,    // server only
  COLSHAPE,         // local (serverside one doesn't exist on clientside)
  CHECKPOINT,       // local (serverside one doesn't exist on clientside?)
  WEBSOCKET_CLIENT, // local
  HTTP_CLIENT,      // local
  AUDIO,            // local

  // local (TODO: what to do when these are destroyed by audio?)
  AUDIO_OUTPUT,
  AUDIO_OUTPUT_WORLD,
  AUDIO_OUTPUT_ATTACHED,
  AUDIO_OUTPUT_FRONTEND,

  RML_ELEMENT,          // .
  RML_DOCUMENT,         // .
  LOCAL_PLAYER,         // local and static lifetime, always exist
  LOCAL_OBJECT,         // local
  VIRTUAL_ENTITY,       // local and server: WORLD_OBJECT_STREAM_IN -> WORLD_OBJECT_STREAM_OUT
  VIRTUAL_ENTITY_GROUP, // local (server one doesn't exist on clientside)
  MARKER,               // local
  TEXT_LABEL,           // local
  LOCAL_PED,            // local
  LOCAL_VEHICLE,        // local
  AUDIO_FILTER,         // local
  CONNECTION_INFO,      // server only
  CUSTOM_TEXTURE,       // what is it?
  FONT,                 // local
  SIZE,
}
