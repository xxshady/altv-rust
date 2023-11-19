#[allow(nonstandard_style)]
pub mod natives_result;

pub type BaseObjectPtr = u64;

pub type BaseObjectTypeRaw = u8;

// TODO: add proper generation of enums from cpp sdk
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum BaseObjectType {
    Player,
    Vehicle,
    Ped,
    Object,
    Blip,
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
    VirtualEntity,
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
}

impl TryFrom<BaseObjectTypeRaw> for BaseObjectType {
    type Error = ();
    fn try_from(v: BaseObjectTypeRaw) -> Result<Self, Self::Error> {
        Ok(match v {
            v if v == Self::Player as BaseObjectTypeRaw => Self::Player,
            v if v == Self::Vehicle as BaseObjectTypeRaw => Self::Vehicle,
            v if v == Self::Ped as BaseObjectTypeRaw => Self::Ped,
            v if v == Self::Object as BaseObjectTypeRaw => Self::Object,
            v if v == Self::Blip as BaseObjectTypeRaw => Self::Blip,
            v if v == Self::Webview as BaseObjectTypeRaw => Self::Webview,
            v if v == Self::VoiceChannel as BaseObjectTypeRaw => Self::VoiceChannel,
            v if v == Self::Colshape as BaseObjectTypeRaw => Self::Colshape,
            v if v == Self::Checkpoint as BaseObjectTypeRaw => Self::Checkpoint,
            v if v == Self::WebsocketClient as BaseObjectTypeRaw => Self::WebsocketClient,
            v if v == Self::HttpClient as BaseObjectTypeRaw => Self::HttpClient,
            v if v == Self::Audio as BaseObjectTypeRaw => Self::Audio,
            v if v == Self::AudioOutput as BaseObjectTypeRaw => Self::AudioOutput,
            v if v == Self::AudioOutputWorld as BaseObjectTypeRaw => Self::AudioOutputWorld,
            v if v == Self::AudioOutputAttached as BaseObjectTypeRaw => Self::AudioOutputAttached,
            v if v == Self::AudioOutputFrontend as BaseObjectTypeRaw => Self::AudioOutputFrontend,
            v if v == Self::RmlElement as BaseObjectTypeRaw => Self::RmlElement,
            v if v == Self::RmlDocument as BaseObjectTypeRaw => Self::RmlDocument,
            v if v == Self::LocalPlayer as BaseObjectTypeRaw => Self::LocalPlayer,
            v if v == Self::LocalObject as BaseObjectTypeRaw => Self::LocalObject,
            v if v == Self::VirtualEntity as BaseObjectTypeRaw => Self::VirtualEntity,
            v if v == Self::VirtualEntityGroup as BaseObjectTypeRaw => Self::VirtualEntityGroup,
            v if v == Self::Marker as BaseObjectTypeRaw => Self::Marker,
            v if v == Self::TextLabel as BaseObjectTypeRaw => Self::TextLabel,
            v if v == Self::LocalPed as BaseObjectTypeRaw => Self::LocalPed,
            v if v == Self::LocalVehicle as BaseObjectTypeRaw => Self::LocalVehicle,
            v if v == Self::AudioFilter as BaseObjectTypeRaw => Self::AudioFilter,
            v if v == Self::ConnectionInfo as BaseObjectTypeRaw => Self::ConnectionInfo,
            v if v == Self::CustomTexture as BaseObjectTypeRaw => Self::CustomTexture,
            v if v == Self::Font as BaseObjectTypeRaw => Self::Font,
            v if v == Self::Size as BaseObjectTypeRaw => Self::Size,
            _ => return Err(()),
        })
    }
}

