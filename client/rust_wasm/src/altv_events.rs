use std::{
  cell::{Cell, RefCell},
  collections::HashMap,
  rc::Rc,
  time::Duration,
};

use crate::{
  async_executor::spawn_future,
  base_objects::handle::RawBaseObjectHandle,
  id_provider::{Id, IdProvider},
  logging::{log_info, log_warn},
  wait::wait,
  wasm_imports::{self, disable_altv_event, enable_altv_event},
};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

macro_rules! define_altv_events {
  ( $( $variant:ident: $payload:tt, )+ ) => {
    #[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
    #[repr(u8)]
    pub enum EventType {
      $( $variant, )+
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Event {
      $( $variant(contexts::$variant), )+
    }

    impl Event {
      fn event_type(&self) -> EventType {
        match self {
          $( Self::$variant(_) => EventType::$variant, )+
        }
      }

      pub fn event_name(&self) -> &'static str {
        match self {
          $( Self::$variant(_) => stringify!($variant), )+
        }
      }
    }

    pub mod contexts {
      use super::*;

      $(
        #[derive(Debug, Serialize, Deserialize)]
        pub struct $variant $payload
      )+
    }

    pub enum Handler {
      $( $variant(Box<dyn FnMut(&contexts::$variant)>), )+
    }

    impl Handler {
      pub fn event_type(&self) -> EventType {
        match self {
          $( Self::$variant(_) => EventType::$variant, )+
        }
      }

      pub fn event_name(&self) -> &'static str {
        match self {
          $( Self::$variant(_) => stringify!($variant), )+
        }
      }

      pub fn call_by_event_type(&mut self, event: &Event) {
        match event { $(
          Event::$variant(payload) => {
            let Self::$variant(callback) = self else {
              panic!("Expected {} handler", stringify!($variant));
            };
            callback(payload);
          }
        )+ }
      }
    }
  };
}

define_altv_events!(
  serverStarted: {},
  consoleCommand: {
    pub name: String,
    pub args: Vec<String>,
  },
  baseObjectCreate: { pub base_object: RawBaseObjectHandle },
  baseObjectRemove: { pub base_object: RawBaseObjectHandle },
  gameEntityCreate: { pub entity: RawBaseObjectHandle },
  gameEntityDestroy: { pub entity: RawBaseObjectHandle },
  worldObjectStreamIn: { pub world_object: RawBaseObjectHandle },
  worldObjectStreamOut: { pub world_object: RawBaseObjectHandle },
);

#[wasm_bindgen]
pub fn on_altv_event(event: JsValue) {
  log_info!("on_altv_event {event:?}");
  let event: Event = from_value(event).unwrap();
  log_info!("on_altv_event {event:?}");

  MANAGER_INSTANCE.with_borrow_mut(|manager| {
    let Some(handlers) = manager.handlers.get_mut(&event.event_type()) else {
      log_warn!("received event: {} without handlers", event.event_name());
      return;
    };

    for handler in handlers.values_mut() {
      handler.call_by_event_type(&event);
    }
  });
}

thread_local! {
  static MANAGER_INSTANCE: RefCell<Manager> = Default::default();
}

type HandlerId = Id;

#[derive(Default)]
struct Manager {
  handlers: HashMap<EventType, HashMap<HandlerId, Handler>>,
  handler_id_provider: IdProvider,
}

// TODO: add IntoAnyVoidResult support
pub fn add_handler(handler: Handler) -> HandlerId {
  MANAGER_INSTANCE.with_borrow_mut(|instance| {
    let per_type_handlers = instance.handlers.entry(handler.event_type()).or_default();

    if per_type_handlers.is_empty() {
      enable_altv_event(handler.event_name());
    }

    let id = instance.handler_id_provider.next();
    per_type_handlers.insert(id, handler);
    id
  })
}

pub fn remove_handler(handler_id: HandlerId) {
  MANAGER_INSTANCE.with_borrow_mut(|instance| {
    let mut remove_map_of_type = None;
    for (event_type, handlers_map) in &mut instance.handlers {
      let Some(handler) = handlers_map.remove(&handler_id) else {
        continue;
      };

      if handlers_map.is_empty() {
        disable_altv_event(handler.event_name());
        remove_map_of_type = Some(*event_type);
      }

      break;
    }

    if let Some(event_type) = remove_map_of_type {
      instance.handlers.remove(&event_type);
    }
  });
}

#[wasm_bindgen]
pub fn test_altv_events() {
  let handler_id2 = Rc::new(Cell::new(None));
  let handler_id2_ = {
    let id = handler_id2.clone();
    add_handler(Handler::consoleCommand(Box::new(move |context| {
      log_info!("context2: {context:?}");
      log_info!("handler_id2: {id:?}");

      if context.name == "kkkk" {
        let id = id.clone();
        spawn_future(async move {
          remove_handler(id.get().unwrap());
        });
      }
    })))
  };
  handler_id2.replace(Some(handler_id2_));
  log_info!("handler_id2: {handler_id2:?}");

  let handler_id = add_handler(Handler::consoleCommand(Box::new(|context| {
    log_info!("context: {context:?}");
  })));

  MANAGER_INSTANCE.with_borrow(|manager| {
    assert!(!manager.handlers.is_empty());
  });

  spawn_future(async move {
    wait(Duration::from_secs(2)).await;
    log_info!("removing handler");
    remove_handler(handler_id);
    MANAGER_INSTANCE.with_borrow(|manager| {
      assert_eq!(
        manager
          .handlers
          .get(&EventType::consoleCommand)
          .unwrap()
          .len(),
        1
      );
    });
  });
}

#[wasm_bindgen]
pub fn test_altv_events2() {
  add_handler(Handler::consoleCommand(Box::new(|context| {
    if context.name != "net_time" {
      return;
    }

    let net_time = Duration::from_millis(wasm_imports::get_net_time().into());
    log_info!("net time ms: {:?}", net_time.as_millis());
    log_info!("net time  s: {:?}", net_time.as_secs());
  })));
}
