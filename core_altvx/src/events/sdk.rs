pub use altv_sdk::EventType as SDKEventType;

macro_rules! supported_cpp_sdk_events {
    ($enum_name: ident [ $( $event: ident ),+ ] ) => {
        #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
        pub enum $enum_name {
            $( $event, )+
        }

        impl TryFrom<SDKEventType> for $enum_name {
            type Error = anyhow::Error;
            fn try_from(value: SDKEventType) -> anyhow::Result<Self> {
                match value {
                    $(
                        SDKEventType::$event => Ok($enum_name::$event),
                    )+
                    event => {
                        anyhow::bail!("unsupported cpp sdk event type: {event:?}")
                    }
                }
            }
        }

        impl Into<SDKEventType> for $enum_name {
            fn into(self) -> SDKEventType {
                match self {
                    $(
                        $enum_name::$event => SDKEventType::$event,
                    )+
                }
            }
        }
    };
}

supported_cpp_sdk_events!(SupportedEventType[ServerStarted, ColshapeEvent, PlayerConnect]);
