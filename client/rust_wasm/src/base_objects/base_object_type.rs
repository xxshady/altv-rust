use serde_repr::{Deserialize_repr, Serialize_repr};

use super::sdk_base_object_type::SdkBaseObjectType;

#[derive(Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum BaseObjectType {
  // keep same order as in sdk enum
  Player,
  Vehicle,
  Ped,
  Object,
  Blip, // server blip
  Webview,
  VoiceChannel,
  Colshape,
  Checkpoint,
  WebsocketClient,
  HttpClient,
  Audio,
  AudioOutput,
  AudioOutputWorld,
  AudioOutputAttached,
  AudioOutputFrontend,
  RmlElement,
  RmlDocument,
  LocalPlayer,
  LocalObject,
  VirtualEntity, // server virtual entity
  VirtualEntityGroup,
  Marker,
  TextLabel,
  LocalPed,
  LocalVehicle,
  AudioFilter,
  ConnectionInfo,
  CustomTexture,
  Font,
  Size,

  // custom: (should come after sdk enum variants!!!)
  LocalBlip,
  LocalVirtualEntity,
}

pub fn sdk_to_rust_base_object_type(sdk: SdkBaseObjectType, is_remote: bool) -> BaseObjectType {
  match (sdk, is_remote) {
    (
      SdkBaseObjectType::SIZE
      | SdkBaseObjectType::VOICE_CHANNEL
      | SdkBaseObjectType::CONNECTION_INFO
      | SdkBaseObjectType::CUSTOM_TEXTURE,
      _,
    ) => unreachable!(),

    (SdkBaseObjectType::BLIP, false) => BaseObjectType::LocalBlip,
    (SdkBaseObjectType::BLIP, true) => BaseObjectType::Blip,
    (SdkBaseObjectType::VIRTUAL_ENTITY, false) => BaseObjectType::LocalVirtualEntity,
    (SdkBaseObjectType::VIRTUAL_ENTITY, true) => BaseObjectType::VirtualEntity,

    (SdkBaseObjectType::PLAYER, _) => BaseObjectType::Player,
    (SdkBaseObjectType::VEHICLE, _) => BaseObjectType::Vehicle,
    (SdkBaseObjectType::PED, _) => BaseObjectType::Ped,
    (SdkBaseObjectType::OBJECT, _) => BaseObjectType::Object,
    (SdkBaseObjectType::WEBVIEW, _) => BaseObjectType::Webview,
    (SdkBaseObjectType::COLSHAPE, _) => BaseObjectType::Colshape,
    (SdkBaseObjectType::CHECKPOINT, _) => BaseObjectType::Checkpoint,
    (SdkBaseObjectType::WEBSOCKET_CLIENT, _) => BaseObjectType::WebsocketClient,
    (SdkBaseObjectType::HTTP_CLIENT, _) => BaseObjectType::HttpClient,
    (SdkBaseObjectType::AUDIO, _) => BaseObjectType::Audio,
    (SdkBaseObjectType::AUDIO_OUTPUT, _) => BaseObjectType::AudioOutput, // TODO: what is it?
    (SdkBaseObjectType::AUDIO_OUTPUT_WORLD, _) => BaseObjectType::AudioOutputWorld,
    (SdkBaseObjectType::AUDIO_OUTPUT_ATTACHED, _) => BaseObjectType::AudioOutputAttached,
    (SdkBaseObjectType::AUDIO_OUTPUT_FRONTEND, _) => BaseObjectType::AudioOutputFrontend,
    (SdkBaseObjectType::RML_ELEMENT, _) => BaseObjectType::RmlElement,
    (SdkBaseObjectType::RML_DOCUMENT, _) => BaseObjectType::RmlDocument,
    (SdkBaseObjectType::LOCAL_PLAYER, _) => BaseObjectType::LocalPlayer,
    (SdkBaseObjectType::LOCAL_OBJECT, _) => BaseObjectType::LocalObject,
    (SdkBaseObjectType::VIRTUAL_ENTITY_GROUP, _) => BaseObjectType::VirtualEntityGroup,
    (SdkBaseObjectType::MARKER, _) => BaseObjectType::Marker,
    (SdkBaseObjectType::TEXT_LABEL, _) => BaseObjectType::TextLabel,
    (SdkBaseObjectType::LOCAL_PED, _) => BaseObjectType::LocalPed,
    (SdkBaseObjectType::LOCAL_VEHICLE, _) => BaseObjectType::LocalVehicle,
    (SdkBaseObjectType::AUDIO_FILTER, _) => BaseObjectType::AudioFilter,
    (SdkBaseObjectType::FONT, _) => BaseObjectType::Font,
  }
}

type IsRemote = bool;
pub fn rust_to_sdk_base_object_type(rust_type: BaseObjectType) -> (SdkBaseObjectType, IsRemote) {
  match rust_type {
    BaseObjectType::ConnectionInfo
    | BaseObjectType::VoiceChannel
    | BaseObjectType::CustomTexture
    | BaseObjectType::Size => unreachable!(),

    BaseObjectType::Blip => (SdkBaseObjectType::BLIP, true),
    BaseObjectType::LocalBlip => (SdkBaseObjectType::BLIP, false),
    BaseObjectType::VirtualEntity => (SdkBaseObjectType::VIRTUAL_ENTITY, true),
    BaseObjectType::LocalVirtualEntity => (SdkBaseObjectType::VIRTUAL_ENTITY, false),

    BaseObjectType::Player => (SdkBaseObjectType::PLAYER, true),
    BaseObjectType::Vehicle => (SdkBaseObjectType::VEHICLE, true),
    BaseObjectType::Ped => (SdkBaseObjectType::PED, true),
    BaseObjectType::Object => (SdkBaseObjectType::OBJECT, true),
    BaseObjectType::Webview => (SdkBaseObjectType::WEBVIEW, false),
    BaseObjectType::Colshape => (SdkBaseObjectType::COLSHAPE, false),
    BaseObjectType::Checkpoint => (SdkBaseObjectType::CHECKPOINT, false),
    BaseObjectType::WebsocketClient => (SdkBaseObjectType::WEBSOCKET_CLIENT, false),
    BaseObjectType::HttpClient => (SdkBaseObjectType::HTTP_CLIENT, false),
    BaseObjectType::Audio => (SdkBaseObjectType::AUDIO, false),
    BaseObjectType::AudioOutput => (SdkBaseObjectType::AUDIO_OUTPUT, false), // TODO: what is it?
    BaseObjectType::AudioOutputWorld => (SdkBaseObjectType::AUDIO_OUTPUT_WORLD, false),
    BaseObjectType::AudioOutputAttached => (SdkBaseObjectType::AUDIO_OUTPUT_ATTACHED, false),
    BaseObjectType::AudioOutputFrontend => (SdkBaseObjectType::AUDIO_OUTPUT_FRONTEND, false),
    BaseObjectType::RmlElement => (SdkBaseObjectType::RML_ELEMENT, false),
    BaseObjectType::RmlDocument => (SdkBaseObjectType::RML_DOCUMENT, false),

    // TODO: add warning if local player gets serialized for metadata or events?
    BaseObjectType::LocalPlayer => (SdkBaseObjectType::LOCAL_PLAYER, true),

    BaseObjectType::LocalObject => (SdkBaseObjectType::LOCAL_OBJECT, false),
    BaseObjectType::VirtualEntityGroup => (SdkBaseObjectType::VIRTUAL_ENTITY_GROUP, false),
    BaseObjectType::Marker => (SdkBaseObjectType::MARKER, false),
    BaseObjectType::TextLabel => (SdkBaseObjectType::TEXT_LABEL, false),
    BaseObjectType::LocalPed => (SdkBaseObjectType::LOCAL_PED, false),
    BaseObjectType::LocalVehicle => (SdkBaseObjectType::LOCAL_VEHICLE, false),
    BaseObjectType::AudioFilter => (SdkBaseObjectType::AUDIO_FILTER, false),
    BaseObjectType::Font => (SdkBaseObjectType::FONT, false),
  }
}
