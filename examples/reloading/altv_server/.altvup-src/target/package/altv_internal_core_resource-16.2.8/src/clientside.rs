use std::cell::Cell;
use altv_sdk::{helpers::get_base_object_type, BaseObjectRawMutPtr, BaseObjectType};
use mvalue::to_mvalue;
use crate::{sdk, helpers::base_ptr_to_raw};

// special key to avoid collisions with user's code
pub const GENERATION_ID_KEY: &str = "&^#altv-rust";

thread_local! {
    pub static CURRENT_GENERATION_ID: Cell<u64> = const { Cell::new(1) };
}

pub fn handle_base_object_creation(raw_ptr: BaseObjectRawMutPtr) {
  let btype = unsafe { get_base_object_type(raw_ptr) };
  logger::debug!("initializing base object type: {btype:?} for clientside");

  init_generation_of_created_base_object(raw_ptr, btype);
}

pub fn handle_base_object_destruction() {
  let current = CURRENT_GENERATION_ID.get();
  let next = current.checked_add(1).unwrap_or_else(|| {
    logger::error!(
      "Base object generation reached u64::MAX.\n\
            Next base object will use non-unique generation.\n\
            Consider opening issue in altv-rust repo: https://github.com/xxshady/altv-rust/issues."
    );
    1
  });
  CURRENT_GENERATION_ID.set(next);
}

fn init_generation_of_created_base_object(raw_ptr: BaseObjectRawMutPtr, btype: BaseObjectType) {
  let generation_id = CURRENT_GENERATION_ID.get();

  use BaseObjectType as B;
  match btype {
    B::Player | B::Vehicle | B::Ped | B::Object => {
      let entity = base_ptr_to_raw!(raw_ptr, entity);
      unsafe {
        sdk::IEntity::SetStreamSyncedMetaData(
          entity,
          GENERATION_ID_KEY,
          to_mvalue(&generation_id).unwrap().get(),
        )
      }
    }
    B::VirtualEntity => {
      let entity = base_ptr_to_raw!(raw_ptr, virtual_entity);
      unsafe {
        sdk::IVirtualEntity::SetStreamSyncedMetaData(
          entity,
          GENERATION_ID_KEY,
          to_mvalue(&generation_id).unwrap().get(),
        )
      }
    }
    B::Blip => unsafe {
      sdk::IBaseObject::SetSyncedMetaData(
        raw_ptr,
        GENERATION_ID_KEY,
        to_mvalue(&generation_id).unwrap().get(),
      )
    },
    B::Checkpoint => {
      let checkpoint = base_ptr_to_raw!(raw_ptr, checkpoint);
      unsafe {
        sdk::ICheckpoint::SetStreamSyncedMetaData(
          checkpoint,
          GENERATION_ID_KEY,
          to_mvalue(&generation_id).unwrap().get(),
        )
      }
    }
    B::ConnectionInfo | B::Marker | B::VirtualEntityGroup | B::Colshape | B::VoiceChannel => {
      logger::debug!("skipping {btype:?}");
    }
    B::Size
    | B::Webview
    | B::RmlElement
    | B::RmlDocument
    | B::LocalPlayer
    | B::LocalObject
    | B::TextLabel
    | B::LocalPed
    | B::LocalVehicle
    | B::Font
    | B::CustomTexture
    | B::Audio
    | B::AudioFilter
    | B::AudioOutput
    | B::AudioOutputAttached
    | B::AudioOutputFrontend
    | B::AudioOutputWorld
    | B::HttpClient
    | B::WebsocketClient => unreachable!(),
  }
}
