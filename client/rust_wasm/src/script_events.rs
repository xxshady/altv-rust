use std::{borrow::Cow, cell::RefCell, collections::HashMap, time::Duration};

use crate::{
  any_void_result::IntoAnyVoidResult,
  async_executor::spawn_future,
  base_objects::{
    any_instances::AnyPlayer, class_traits::vehicle::Vehicle, handle::BaseObjectHandle,
    scope::Scope,
  },
  id_provider::{Id, IdProvider},
  logging::{dbg, log_error, log_info, log_warn},
  timers::{set_interval, set_timeout},
  wait::{wait, wait_for},
  wasm_imports,
};

use js_sys::Uint8Array;
use serde::{de::DeserializeOwned, Deserialize};
use serde_bytes::ByteBuf;
use serde_repr::Deserialize_repr;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

pub struct RawScriptEventContext {
  args: RawScriptEventArgs,
}

#[derive(Debug)]
pub struct RawScriptEventArgs {
  raw: JsValue,
}

impl RawScriptEventArgs {
  /// For Rust-to-Rust deserialization
  ///
  /// See also: [`Self::deserialize_js`]
  pub fn deserialize<T>(&self) -> Result<T, DeserializationError>
  where
    T: DeserializeOwned,
  {
    let raw = self.raw.clone();

    let (bytes,): (ByteBuf,) = from_value(raw).map_err(DeserializationError::Serde)?;

    // TODO: let user decide what crate to use for bytes de(serialization)
    bincode::deserialize(&bytes).map_err(|err| DeserializationError::Bincode(*err))
  }

  /// For JS-to-Rust deserialization
  ///
  /// See also: [`Self::deserialize`]
  pub fn deserialize_js<T>(&self) -> Result<T, serde_wasm_bindgen::Error>
  where
    T: DeserializeOwned,
  {
    from_value(self.raw.clone())
  }
}

impl Scope for RawScriptEventContext {}

#[derive(Debug)]
pub enum DeserializationError {
  Serde(serde_wasm_bindgen::Error),
  Bincode(bincode::ErrorKind),
}

pub struct ScriptEventContext<'scope, T: DeserializeOwned> {
  pub data: T,

  /// Prevent creation of this struct from outside and moving out of the handler (because it's Scope)
  _private: &'scope (),
}

impl<'scope, T: DeserializeOwned> Scope for ScriptEventContext<'scope, T> {}

thread_local! {
  static MANAGER_INSTANCE: RefCell<Manager<'static>> = Default::default();
}

type Handler = Box<dyn FnMut(&RawScriptEventContext)>;
type HandlerId = Id;
type HandlersStore<'a> = HashMap<Cow<'a, str>, HashMap<HandlerId, Handler>>;

#[derive(Default)]
struct Manager<'a> {
  local_handlers: HandlersStore<'a>,
  remote_handlers: HandlersStore<'a>,
  handler_id_provider: IdProvider,
}

impl Manager<'_> {
  fn add_handler(
    &mut self,
    kind: EventKind,
    event_name: impl Into<Cow<'static, str>>,
    handler: impl FnMut(&RawScriptEventContext) + 'static,
  ) -> HandlerId {
    let all_handlers = match kind {
      EventKind::Local => &mut self.local_handlers,
      EventKind::Remote => &mut self.remote_handlers,
    };

    let event_name = event_name.into();
    let handlers_of_event = all_handlers.entry(event_name).or_default();
    let id = self.handler_id_provider.next();
    handlers_of_event.insert(id, Box::new(handler));
    id
  }
}

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
enum EventKind {
  Local,
  Remote,
}

#[derive(Debug, Deserialize)]
struct Event {
  source: EventKind,
  name: String,
  #[serde(with = "serde_wasm_bindgen::preserve")]
  args: JsValue,
}

#[wasm_bindgen]
pub fn on_script_event(event: JsValue) {
  let event: Event = from_value(event).unwrap();
  log_info!("on_script_event {event:?}");

  MANAGER_INSTANCE.with_borrow_mut(|instance| {
    let event_name: Cow<'_, str> = event.name.into();

    let handlers = match event.source {
      EventKind::Local => instance.local_handlers.get_mut(&event_name),
      EventKind::Remote => instance.remote_handlers.get_mut(&event_name),
    };
    let Some(handlers) = handlers else {
      log_warn!(
        "received {:?} event: '{}', but no handler was set for it",
        event.source,
        event_name,
      );
      return;
    };

    let context = RawScriptEventContext {
      args: RawScriptEventArgs { raw: event.args },
    };
    for handler in handlers.values_mut() {
      handler(&context);
    }
  });
}

fn wrap_handler_with_deserializer<T: DeserializeOwned>(
  event_name: Cow<'static, str>,
  mut handler: impl FnMut(ScriptEventContext<T>),
) -> impl FnMut(&RawScriptEventContext) {
  move |context| match context.args.deserialize::<T>() {
    Ok(data) => {
      let context = ScriptEventContext {
        data,
        _private: &(),
      };
      handler(context)
    }
    Err(err) => {
      log_error!("Failed to deserialize data for event: {event_name}, error: {err:?}");
    }
  }
}

pub fn add_local_handler_raw<R>(
  event_name: impl Into<Cow<'static, str>>,
  mut handler: impl FnMut(&RawScriptEventContext) -> R + 'static,
) -> HandlerId
where
  R: IntoAnyVoidResult,
{
  MANAGER_INSTANCE.with_borrow_mut(move |instance| {
    instance.add_handler(EventKind::Local, event_name, move |context| {
      let result = handler(context).into_any_void_result();
      let Err(err) = result else {
        return;
      };
      log_error!("raw local event handler returned an error: {err:?}");
    })
  })
}

pub fn add_local_handler<T, R>(
  event_name: impl Into<Cow<'static, str>>,
  mut handler: impl FnMut(ScriptEventContext<T>) -> R + 'static,
) -> HandlerId
where
  T: DeserializeOwned + 'static,
  R: IntoAnyVoidResult,
{
  let event_name = event_name.into();

  let handler = wrap_handler_with_deserializer(event_name.clone(), move |context| {
    let result = handler(context).into_any_void_result();
    let Err(err) = result else {
      return;
    };
    log_error!("local event handler returned an error: {err:?}");
  });

  MANAGER_INSTANCE
    .with_borrow_mut(move |instance| instance.add_handler(EventKind::Local, event_name, handler))
}

// TODO:
// pub fn remove_local_handler(handler_id: HandlerId) {
//   todo!()
// }

pub fn add_remote_handler_raw<R>(
  event_name: impl Into<Cow<'static, str>>,
  mut handler: impl FnMut(&RawScriptEventContext) -> R + 'static,
) -> HandlerId
where
  R: IntoAnyVoidResult,
{
  MANAGER_INSTANCE.with_borrow_mut(move |instance| {
    instance.add_handler(EventKind::Remote, event_name, move |context| {
      let result = handler(context).into_any_void_result();
      let Err(err) = result else {
        return;
      };
      log_error!("raw remote event handler returned an error: {err:?}");
    })
  })
}

pub fn add_remote_handler<T, R>(
  event_name: impl Into<Cow<'static, str>>,
  mut handler: impl FnMut(ScriptEventContext<T>) -> R + 'static,
) -> HandlerId
where
  T: DeserializeOwned + 'static,
  R: IntoAnyVoidResult,
{
  let event_name = event_name.into();

  let handler = wrap_handler_with_deserializer(event_name.clone(), move |context| {
    let result = handler(context).into_any_void_result();
    let Err(err) = result else {
      return;
    };
    log_error!("remote event handler returned an error: {err:?}");
  });

  MANAGER_INSTANCE
    .with_borrow_mut(move |instance| instance.add_handler(EventKind::Remote, event_name, handler))
}

// TODO:
// pub fn remove_remove_handler(handler_id: HandlerId) {
//   todo!()
// }

/// For Rust-to-Rust serialization
///
/// See also: [`emit_js`]
pub fn emit(
  event_name: &str,
  data: &dyn erased_serde::Serialize,
) -> Result<(), bincode::ErrorKind> {
  let bytes = bincode::serialize(data).map_err(|err| *err)?;
  let array = Uint8Array::from(bytes.as_slice());
  wasm_imports::emit_local_event_rust(event_name, array.buffer());
  Ok(())
}

/// For JS-to-Rust serialization
///
/// See also: [`emit`]
pub fn emit_js(
  event_name: &str,
  args: &[&dyn erased_serde::Serialize],
) -> Result<(), serde_wasm_bindgen::Error> {
  let js_value = to_value(args)?;

  wasm_imports::emit_local_event_js(event_name, js_value);

  Ok(())
}

#[wasm_bindgen]
pub fn test_script_events() {
  spawn_future(async move {
    add_remote_handler("test", |ctx: ScriptEventContext<(u8, bool)>| {
      log_info!("remote test data: {:?}", ctx.data);
    });

    let event_name = String::from("test");

    type TestData<'a> = (i32, bool, Vec<i32>);

    let context_: std::rc::Rc<RefCell<Option<ScriptEventContext<(i32, bool, Vec<i32>)>>>> =
      std::rc::Rc::new(RefCell::new(None));

    add_local_handler(event_name.clone(), {
      let _context = context_.clone();
      move |context: ScriptEventContext<TestData>| {
        // let data = context.data;
        // log_info!("rust data: {:?}", data);

        // use of moved value as expected
        // (context.data, context.data);

        // must not be possible because ScriptEventContext is Scope
        // _context.replace(Some(context));
      }
    });

    set_interval(
      move |_| {
        let Some(context) = &*context_.borrow() else {
          return;
        };
        log_info!("{:?}", context.data);
      },
      Duration::from_secs(1),
    );

    fn _test() {
      add_local_handler::<i32, _>("test", |context| {
        log_info!("rust data: {:?}", context.data);
      });
    }

    let data: TestData = (i32::MAX, true, vec![1, 2, 3]);
    emit(&event_name, &data).unwrap();

    wait(Duration::from_millis(500)).await;

    let js_event_name = event_name + "_js";

    add_local_handler_raw(js_event_name.clone(), |context| {
      let (args,): (TestData,) = context.args.deserialize_js().unwrap();
      log_info!("js args: {args:?}");
    });

    emit_js(&js_event_name, &[&data]).unwrap();
  });

  // use crate::base_objects::player::{PlayerHandle, Player};
  // fn handler(context: ScriptEventContext<PlayerHandle>) {
  //   let Some(player) = Player::get_by_handle(&context, context.data) else {
  //     return;
  //   };
  //   log_info!("player.name(): {}", player.name());
  // }
  // add_local_handler("deserialize_base_object", handler);
  // add_remote_handler("deserialize_base_object", handler);

  use crate::base_objects::{
    vehicle::VehicleHandle,
    class_traits::{
      entity::Entity, world_object::WorldObject, entity::SyncedEntity, player::Player,
    },
    handle::AnyBaseObjectHandle,
  };
  fn handler(context: ScriptEventContext<AnyBaseObjectHandle>) {
    let AnyBaseObjectHandle::Vehicle(vehicle_handle) = context.data else {
      unreachable!();
    };

    spawn_future(async move {
      let spawned = wait_for(
        move |scope| {
          if vehicle_handle.attach_to(scope).is_none() {
            return false;
          }

          set_interval(
            move |scope| {
              dbg!(vehicle_handle);
              let vehicle = vehicle_handle.attach_to(scope).unwrap();
              dbg!(vehicle.net_owner());

              // let Some(net_owner) = vehicle.net_owner() else {
              //   log_info!("no net owner");
              //   return;
              // };

              // wont work
              // drop(vehicle);
              // dbg!(net_owner);
            },
            Duration::from_secs(1),
          );

          true
        },
        Duration::from_millis(5000),
      )
      .await;
      if !spawned {
        log_error!("failed to wait for spawn");
        return;
      }
    });
  }
  add_local_handler("deserialize_base_object", handler);
  add_remote_handler("deserialize_base_object", handler);
}
