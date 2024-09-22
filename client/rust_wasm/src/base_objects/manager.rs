use std::cell::RefCell;

use crate::{
  altv_events::{self, EventType},
  log_info,
  logging::log_warn,
};

use super::{handle::GenericBaseObjectHandle, base_object_type::BaseObjectType};

thread_local! {
  pub(crate) static MANAGER_INSTANCE: RefCell<Manager> = Default::default();
}

#[derive(Default)]
pub struct Manager {
  instances: Vec<GenericBaseObjectHandle>,
}

impl Manager {
  pub fn on_create(&mut self, base_object: GenericBaseObjectHandle) {
    self.instances.push(base_object);
  }

  pub fn on_destroy(&mut self, base_object: GenericBaseObjectHandle) {
    let idx =
      self.instances.iter().enumerate().find_map(
        |(idx, el)| {
          if *el == base_object {
            Some(idx)
          } else {
            None
          }
        },
      );

    let Some(idx) = idx else {
      log_warn!("[on_destroy] failed to remove base object: {base_object:?}");
      return;
    };

    self.instances.swap_remove(idx);
  }

  pub fn is_handle_valid(&self, handle: &GenericBaseObjectHandle) -> bool {
    self.instances.iter().any(|el| el == handle)
  }
}

pub fn init() {
  log_info!("initializing base object manager");

  altv_events::add_handler(altv_events::Handler::baseObjectCreate(Box::new(|ctx| {
    MANAGER_INSTANCE.with_borrow_mut(|manager| {
      let handle = ctx.base_object.as_handle();
      if !is_it_creation_or_destruction_event(handle.btype, EventType::baseObjectCreate) {
        return;
      }

      log_info!("[baseObjectCreate] creation of {handle:?}");

      manager.on_create(handle);
    });
  })));

  altv_events::add_handler(altv_events::Handler::baseObjectRemove(Box::new(|ctx| {
    MANAGER_INSTANCE.with_borrow_mut(|manager| {
      let handle = ctx.base_object.as_handle();
      if !is_it_creation_or_destruction_event(handle.btype, EventType::baseObjectRemove) {
        return;
      }

      log_info!("[baseObjectDestroy] destruction of {handle:?}");

      manager.on_destroy(handle);
    });
  })));

  altv_events::add_handler(altv_events::Handler::gameEntityCreate(Box::new(|ctx| {
    MANAGER_INSTANCE.with_borrow_mut(|manager| {
      let handle = ctx.entity.as_handle();
      if !is_it_creation_or_destruction_event(handle.btype, EventType::gameEntityCreate) {
        return;
      }

      log_info!("[gameEntityCreate] creation of {handle:?}");

      manager.on_create(handle);
    });
  })));

  altv_events::add_handler(altv_events::Handler::gameEntityDestroy(Box::new(|ctx| {
    MANAGER_INSTANCE.with_borrow_mut(|manager| {
      let handle = ctx.entity.as_handle();
      if !is_it_creation_or_destruction_event(handle.btype, EventType::gameEntityDestroy) {
        return;
      }

      log_info!("[gameEntityDestroy] destruction of {handle:?}");

      manager.on_destroy(handle);
    });
  })));

  altv_events::add_handler(altv_events::Handler::worldObjectStreamIn(Box::new(|ctx| {
    MANAGER_INSTANCE.with_borrow_mut(|manager| {
      let handle = ctx.world_object.as_handle();
      if !is_it_creation_or_destruction_event(handle.btype, EventType::worldObjectStreamIn) {
        return;
      }

      log_info!("[worldObjectStreamIn] creation of {handle:?}");

      manager.on_create(handle);
    });
  })));

  altv_events::add_handler(altv_events::Handler::worldObjectStreamOut(Box::new(
    |ctx| {
      MANAGER_INSTANCE.with_borrow_mut(|manager| {
        let handle = ctx.world_object.as_handle();
        if !is_it_creation_or_destruction_event(handle.btype, EventType::worldObjectStreamOut) {
          return;
        }

        log_info!("[worldObjectStreamOut] destruction of {handle:?}");

        manager.on_destroy(handle);
      });
    },
  )));
}

fn is_it_creation_or_destruction_event(btype: BaseObjectType, event: EventType) -> bool {
  use BaseObjectType as B;
  match btype {
    B::LocalPlayer => false, // TODO: make it unreachable?
    B::Size | B::VoiceChannel | B::ConnectionInfo | B::CustomTexture => unreachable!(),
    // server
    B::Player | B::Vehicle | B::Ped | B::Object | B::Blip | B::VirtualEntity => matches!(
      event,
      EventType::gameEntityCreate | EventType::gameEntityDestroy
    ),
    // local
    B::Webview
    | B::Colshape
    | B::WebsocketClient
    | B::HttpClient
    | B::Audio
    | B::AudioOutput
    | B::AudioOutputWorld
    | B::AudioOutputAttached
    | B::AudioOutputFrontend
    | B::RmlElement
    | B::RmlDocument
    | B::LocalObject
    | B::VirtualEntityGroup // not sent from server
    | B::Marker // currently disabled on server
    | B::TextLabel
    | B::LocalPed
    | B::LocalVehicle
    | B::AudioFilter
    | B::Font | B::LocalBlip | B::LocalVirtualEntity | B::Checkpoint => {
      matches!(
        event,
        EventType::baseObjectCreate | EventType::baseObjectRemove
      )
    }
  }
}
