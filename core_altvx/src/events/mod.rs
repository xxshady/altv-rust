use std::{fmt::Debug, collections::HashMap};
use crate::{resource::Resource, VoidResult, SomeResult};

pub use altv_sdk::EventType as SDKEventType;

pub mod sdk_controllers;
pub mod custom_controllers;
pub(self) mod helpers;

macro_rules! log_user_handler_error {
    ($event_name: expr, $result: expr) => {
        if let Err(err) = $result {
            logger::error!("handler of event {:?} failed with error: {:?}", stringify!($event_name), err);
        }
    };
}

macro_rules! supported_sdk_events {
    ( $( $event_name: ident, )+ ) => {
        #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
        pub enum SupportedEventType {
            $( $event_name, )+
        }

        impl TryFrom<SDKEventType> for SupportedEventType {
            type Error = anyhow::Error;
            fn try_from(value: SDKEventType) -> SomeResult<Self> {
                match value {
                    $(
                        SDKEventType::$event_name => Ok(Self::$event_name),
                    )+
                    event => {
                        anyhow::bail!("unsupported cpp sdk event type: {event:?}")
                    }
                }
            }
        }

        #[allow(clippy::from_over_into)]
        impl Into<SDKEventType> for SupportedEventType {
            fn into(self) -> SDKEventType {
                match self { $(
                    Self::$event_name => SDKEventType::$event_name,
                )+ }
            }
        }

        pub enum SDKController {
            $( $event_name(sdk_controllers::$event_name), )+
        }

        impl std::fmt::Debug for SDKController {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let str = format!("{}", match self {
                    $( Self::$event_name(_) => stringify!(SDKController::$event_name), )+
                });

                f.write_str(&str)
            }
        }

        pub enum SDKHandler { $(
            $event_name(Box<dyn FnMut(&sdk_controllers::$event_name) -> VoidResult + 'static>),
        )+ }

        impl SDKHandler {
            pub fn to_event_type(&self) -> SupportedEventType {
                match self { $(
                    Self::$event_name(_) => SupportedEventType::$event_name,
                )+ }
            }
        }
        
        impl std::fmt::Debug for SDKHandler {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let str = format!("{}", match self {
                    $( Self::$event_name(_) => stringify!(SDKHandler::$event_name), )+
                });

                f.write_str(&str)
            }
        }

        pub(crate) fn sdk_controller_from_supported_event_type(
            event_type: SupportedEventType, 
            event_ptr: altv_sdk::CEventPtr,
            resource: &Resource,
        ) -> SDKController {
            match event_type { $(
                SupportedEventType::$event_name => 
                    SDKController::$event_name(unsafe { sdk_controllers::$event_name::new(event_ptr, resource) }),
            )+ }
        }

        pub fn call_user_sdk_handlers(controller: &SDKController, handlers: &mut [SDKHandler]) {
            for h in handlers {
                match h { $(
                    SDKHandler::$event_name(h) => {
                        let result = h(
                            if let SDKController::$event_name(controller) = controller {
                                controller
                            } else {
                                // this should never happen because SDKHandler gets converted to SupportedEventType
                                // automatically with `to_event_type()`
                                panic!("expected SDKController: {}, received: {controller:?}", stringify!($event_name))
                            }
                        );
                        log_user_handler_error!($event_name, result);
                    }
                )+ }
            }
        }
    }
}

macro_rules! custom_events {
    ( $(
        $sdk_event_name: ident: [ $( $custom_event_name: ident, )+ ],
    )+ ) => {
        #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
        pub enum CustomEventType { $($(
            $custom_event_name,
        )+)+ }

        #[allow(clippy::from_over_into)]
        impl Into<SupportedEventType> for CustomEventType {
            fn into(self) -> SupportedEventType {
                match self { $($(
                        Self::$custom_event_name => SupportedEventType::$sdk_event_name,
                )+)+ }
            }
        }

        pub enum CustomController { $($(
            $custom_event_name(custom_controllers::$custom_event_name),
        )+)+ }
        
        impl std::fmt::Debug for CustomController {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let str = format!("{}", match self {
                    $($( Self::$custom_event_name(_) => stringify!(CustomController::$custom_event_name), )+)+
                });

                f.write_str(&str)
            }
        }

        pub enum CustomHandler { $($(
            $custom_event_name(Box<dyn FnMut(&custom_controllers::$custom_event_name) -> VoidResult + 'static>),
        )+)+ }

        impl CustomHandler {
            pub fn to_event_type(&self) -> CustomEventType {
                match self { $($(
                    CustomHandler::$custom_event_name(_) => CustomEventType::$custom_event_name,
                )+)+ }
            }
        }

        impl std::fmt::Debug for CustomHandler {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let str = format!("{}", match self {
                    $($( Self::$custom_event_name(_) => stringify!(CustomHandler::$custom_event_name), )+)+
                });

                f.write_str(&str)
            }
        }

        pub fn custom_controller_from_event_type(
            event_type: CustomEventType,
            controller: &SDKController,
            resource: &Resource,
        ) -> Option<CustomController> {
            match (controller, event_type) {
                $($(
                    (SDKController::$sdk_event_name(controller), CustomEventType::$custom_event_name) => {
                        if let Some(c) = custom_controllers::$custom_event_name::new(controller, resource) {
                            Some(CustomController::$custom_event_name(c))
                        } else {
                            None
                        }
                    }
                )+)+
                _ => None,
            }
        }

        pub fn call_user_custom_handlers(controller: &CustomController, handlers: &mut [CustomHandler]) {
            for h in handlers {
                match h { $($(
                    CustomHandler::$custom_event_name(h) => {
                        let result = h(
                            if let CustomController::$custom_event_name(controller) = controller {
                                controller
                            } else {
                                // this shit should never happen
                                panic!("expected CustomController: {}, received: {controller:?}", stringify!($custom_event_name))
                            }
                        );
                        log_user_handler_error!($custom_event_name, result);
                    }
                )+)+ }
            }
        }

        pub fn get_custom_event_types_from_sdk_type<'a>(sdk_event_type: SupportedEventType) -> Option<&'a[CustomEventType]> {
            match sdk_event_type {
            $(
                SupportedEventType::$sdk_event_name => Some(&[$(
                    CustomEventType::$custom_event_name,
                )+]),
            )+
            _ => None,
         }
        }
    }
}

supported_sdk_events!(
    ServerStarted,
    PlayerConnect,
    PlayerDisconnect,
    ColshapeEvent,
    ServerScriptEvent,
    ClientScriptEvent,
    ConsoleCommandEvent,
    WeaponDamageEvent,
    PlayerDeath,
    PlayerDamage,
    PlayerEnteringVehicle,
    PlayerEnterVehicle,
    PlayerLeaveVehicle,
    PlayerChangeVehicleSeat,
    PlayerWeaponChange,
);

custom_events!(
    ColshapeEvent: [
        VehicleEnterColShape,
        VehicleLeaveColShape,
        PlayerEnterColShape,
        PlayerLeaveColShape,
    ],
);

#[derive(Default, Debug)]
pub struct EventManager {
    user_sdk_handlers: HashMap<SupportedEventType, Vec<SDKHandler>>,
    user_custom_handlers: HashMap<CustomEventType, Vec<CustomHandler>>,
}

impl EventManager {
    pub fn on_sdk_event(
        &mut self,
        sdk_event_type: SDKEventType,
        event: altv_sdk::CEventPtr,
        resource: &Resource,
    ) {
        match SupportedEventType::try_from(sdk_event_type) {
            Err(err) => logger::error!("{:?}", err),
            Ok(event_type) => self.on_supported_sdk_event(event_type, event, resource),
        };
    }

    pub fn on_supported_sdk_event(&mut self, event_type: SupportedEventType, event_ptr: altv_sdk::CEventPtr, resource: &Resource) {
        let controller = sdk_controller_from_supported_event_type(event_type, event_ptr, resource);

        if let Some(handlers) = self.user_sdk_handlers.get_mut(&event_type) {
            call_user_sdk_handlers(&controller, handlers);
        } else {
            logger::debug!("no user sdk handlers for event: {event_type:?}");
        }
        
        self.handle_custom_event_type(event_type, controller, resource);
    }

    fn handle_custom_event_type(&mut self, event_type: SupportedEventType, controller: SDKController, resource: &Resource) {
        let Some(custom_types) = get_custom_event_types_from_sdk_type(event_type) else {
            logger::debug!("no custom sdk handlers for event: {event_type:?}");
            return;
        };

        for custom_type in custom_types {
            let Some(handlers) = self.user_custom_handlers.get_mut(custom_type) else {
                logger::debug!("user sdk event: {event_type:?} is unhandled");
                continue;
            };

            let controller = custom_controller_from_event_type(*custom_type, &controller, resource);
            let Some(controller) = controller else {
                logger::debug!("custom event: {custom_type:?} controller does not exist or event should not be called now");
                continue;  
            };

            call_user_custom_handlers(&controller, handlers);
        }
    }

    pub fn add_sdk_handler(&mut self, handler: SDKHandler) {
        let event_type = handler.to_event_type();
        self.user_sdk_handlers
            .entry(event_type)
            .or_default()
            .push(handler);

        self.toggle_sdk_event(event_type, true);
    }

    pub fn add_custom_handler(&mut self, handler: CustomHandler) {
        let custom_event_type = handler.to_event_type();

        self.user_custom_handlers
            .entry(custom_event_type)
            .or_insert(vec![])
            .push(handler);

        self.toggle_sdk_event(custom_event_type.into(), true);
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

pub fn add_sdk_handler(handler: SDKHandler) {
    Resource::with_events_mut(|mut events, _| {
        events.add_sdk_handler(handler);
    });
}

pub fn add_custom_handler(handler: CustomHandler) {
    Resource::with_events_mut(|mut events, _| {
        events.add_custom_handler(handler);
    });
}
