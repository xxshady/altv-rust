use std::collections::HashMap;

mod controllers;
mod parsers;
mod sdk;

use crate::resource::Resource;
use parsers::{CustomEvent, SDKEvent};
use sdk::SDKEventType;

pub use sdk::SupportedEventType;

macro_rules! event_parser_enum {
    ($parser_enum_name: ident, $event_type_enum: ty, $parser_trait: ty, $handler_enum_name: ident [ $( $parser_name: ident ),+ ]) => {
        enum $parser_enum_name {
            $( $parser_name(parsers::$parser_name), )+
        }

        impl $parser_trait for $parser_enum_name {
            fn handle(&self, event: altv_sdk::CEventPtr, handlers: &mut [$handler_enum_name], resource: &Resource) {
                match self {
                    $(
                        Self::$parser_name(parser) => parser.handle(event, handlers, resource),
                    )+
                }
            }
        }

        impl From<$event_type_enum> for $parser_enum_name {
            fn from(value: $event_type_enum) -> Self {
                match value {
                    $(
                        <$event_type_enum>::$parser_name => Self::$parser_name(parsers::$parser_name {}),
                    )+
                }
            }
        }

        pub enum $handler_enum_name {
            $( $parser_name(Box<dyn FnMut(&controllers::$parser_name)>), )+
        }

        impl std::fmt::Debug for $handler_enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let str = format!("{}", match self {
                    $( $handler_enum_name::$parser_name(_) => stringify!($handler_enum_name::$parser_name), )+
                });

                f.write_str(&str)
            }
        }
    };
}

macro_rules! custom_events {
    ($event_type_enum_name: ident, $parser_enum_name: ident, $parser_trait: ty, $handler_enum_name: ident [ $( $parser_name: ident ),+ ] ) => {
        #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
        pub enum $event_type_enum_name {
            $( $parser_name, )+
        }

        event_parser_enum!(
            $parser_enum_name,
            $event_type_enum_name,
            $parser_trait,
            $handler_enum_name
            [
                $( $parser_name ),+
            ]
        );
    };
}

event_parser_enum!(
    SDKParsers,
    SupportedEventType,
    parsers::SDKEvent,
    SDKHandler[ServerStarted, ColshapeEvent, PlayerConnect]
);

custom_events!(
    CustomEventType,
    CustomParsers,
    parsers::CustomEvent,
    CustomHandler[VehicleEnterColShape]
);

#[derive(Default, Debug)]
pub struct EventManager {
    handled_custom_sdk_events: HashMap<SupportedEventType, CustomEventType>,
    user_sdk_handlers: HashMap<SupportedEventType, Vec<SDKHandler>>,
    user_custom_handlers: HashMap<CustomEventType, Vec<CustomHandler>>,
}

// TODO: event cancellation implementation + player damage event SetDamageValue!!
impl EventManager {
    pub fn on_sdk_event(
        &mut self,
        sdk_event_type: SDKEventType,
        event: altv_sdk::CEventPtr,
        resource: &Resource,
    ) {
        match SupportedEventType::try_from(sdk_event_type) {
            Err(err) => logger::error!("{:?}", err),
            Ok(event_type) => self.on_supported_event(event_type, event, resource),
        };
    }

    fn on_supported_event(
        &mut self,
        event_type: SupportedEventType,
        event: altv_sdk::CEventPtr,
        resource: &Resource,
    ) {
        let parser = SDKParsers::from(event_type);
        if let Some(handlers) = self.user_sdk_handlers.get_mut(&event_type) {
            parser.handle(event, handlers, resource);
        } else {
            println!("no user sdk handlers for event: {event_type:?}");
        }

        if let Some(custom_type) = self.handled_custom_sdk_events.get(&event_type) {
            let parser = CustomParsers::from(*custom_type);

            if let Some(handlers) = self.user_custom_handlers.get_mut(custom_type) {
                parser.handle(event, handlers, resource);
            } else {
                logger::error!("no user custom handlers for custom event: {custom_type:?}");
            }
        } else {
            println!("no custom sdk handlers for event: {event_type:?}");
        }
    }

    // TODO: toggle sdk event
    pub fn add_sdk_handler(&mut self, event_type: SupportedEventType, handler: SDKHandler) {
        self.user_sdk_handlers
            .entry(event_type)
            .or_insert(vec![])
            .push(handler);

        self.toggle_sdk_event(event_type, true);
    }

    // TODO: toggle sdk event
    pub fn add_custom_handler(
        &mut self,
        custom_event_type: CustomEventType,
        handler: CustomHandler,
    ) {
        self.user_custom_handlers
            .entry(custom_event_type)
            .or_insert(vec![])
            .push(handler);

        let mut enable_event = |event_type, custom_event_type| {
            self.handled_custom_sdk_events
                .entry(event_type)
                .or_insert(custom_event_type);
            self.toggle_sdk_event(event_type, true);
        };

        match custom_event_type {
            CustomEventType::VehicleEnterColShape => {
                enable_event(
                    SupportedEventType::ServerStarted,
                    CustomEventType::VehicleEnterColShape,
                );
            }
        }
    }

    fn toggle_sdk_event(&self, event_type: SupportedEventType, state: bool) {
        Resource::with(|r| {
            (r.module_handlers.toggle_event_type)(
                r.full_main_path.clone(),
                event_type.into(),
                state,
            );
        });
    }
}

pub fn add_handler(event_type: SupportedEventType, handler: SDKHandler) {
    Resource::with_events_mut(|mut events, _| {
        events.add_sdk_handler(event_type, handler);
    });
}

// pub fn add_extra_handler(
//     public_event: Event,
//     extra_type: ExtraEventType,
//     sdk_type: SDKEventType,
//     event: ExtraEvent,
// ) {
//     Resource::with_events_mut(|mut events, _| {
//         events.add_extra_handler(public_event, extra_type, sdk_type, event);
//     });
// }
