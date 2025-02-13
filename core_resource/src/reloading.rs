use altv_sdk::BaseObjectRawMutPtr;

use crate::{
  events::{self, SDKHandler},
  base_objects::AnyBaseObject,
  resource::Resource,
  VoidResult,
};

#[derive(Debug, Default)]
pub struct Manager {
  base_objects: Vec<BaseObjectRawMutPtr>,
}

pub fn init() {
  events::add_sdk_handler(SDKHandler::ResourceStop(Box::new(|_| {
    Resource::with_reloading_mut(|manager, _| {
      let all = crate::base_object_funcs::all();

      logger::debug!("all base objects: {}", all.len());

      for base_obj in all {
        let ptr = base_obj.raw_base_ptr()?;
        if manager.base_objects.contains(&ptr) {
          destroy_base_object(base_obj)?;
        }
      }
      Ok(())
    })
  })));
}

pub fn handle_base_object_creation(ptr: BaseObjectRawMutPtr) {
  Resource::with_reloading_mut(|mut manager, _| {
    manager.base_objects.push(ptr);
  });
}

pub fn handle_base_object_destruction(ptr: BaseObjectRawMutPtr) {
  Resource::with(|resource| {
    let Ok(mut manager) = resource.reloading.try_borrow_mut() else {
      logger::debug!("[handle_base_object_destruction] failed to borrow manager");
      return;
    };

    let idx = manager.base_objects.iter().position(|&ptr_| ptr_ == ptr);
    let Some(idx) = idx else {
      return;
    };
    manager.base_objects.swap_remove(idx);
  });
}

fn destroy_base_object(base_obj: AnyBaseObject) -> VoidResult {
  match base_obj {
    AnyBaseObject::Blip(o) => {
      o.destroy()?;
    }
    AnyBaseObject::Checkpoint(o) => {
      o.destroy()?;
    }
    AnyBaseObject::ColShape(o) => {
      o.destroy()?;
    }
    AnyBaseObject::ConnectionInfo(_) => {}
    AnyBaseObject::Marker(o) => {
      o.destroy()?;
    }
    AnyBaseObject::Object(o) => {
      o.destroy()?;
    }
    AnyBaseObject::Vehicle(o) => {
      o.destroy()?;
    }
    AnyBaseObject::Player(_) => {}
    AnyBaseObject::Ped(o) => {
      o.destroy()?;
    }
    AnyBaseObject::VirtualEntity(o) => {
      o.destroy()?;
    }
    AnyBaseObject::VirtualEntityGroup(_) => {}
    AnyBaseObject::VoiceChannel(o) => {
      o.destroy()?;
    }
  };

  Ok(())
}
