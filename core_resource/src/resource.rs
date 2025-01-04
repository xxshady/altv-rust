use std::{
  cell::{Ref, RefCell, RefMut},
  rc::Rc,
  sync::{Arc, LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use core_shared::{ModuleHandlers, StringResourceName};

use crate::{alt_resource, base_objects, events, script_events, timers};

pub static RESOURCE: LazyLock<Arc<RwLock<Option<Resource>>>> =
  LazyLock::new(|| Arc::new(RwLock::new(None)));

#[derive(Debug, Default)]
pub struct Resource {
  pub name: StringResourceName,
  pub module_handlers: ModuleHandlers,

  pub timers: RwLock<timers::TimerManager>,
  pub timer_schedule: RwLock<timers::ScheduleState>,
  pub events: RwLock<events::EventManager>,
  pub local_script_events: RwLock<script_events::LocalEventManager>,
  pub local_script_events_schedule: RwLock<script_events::LocalEventSchedule>,
  pub client_script_events: RwLock<script_events::ClientEventManager>,
  pub client_script_events_schedule: RwLock<script_events::ClientEventSchedule>,
  pub base_objects: RwLock<base_objects::Store>,
  pub pending_base_object_destroy_or_creation: RwLock<base_objects::PendingDestroyOrCreation>,
  pub alt_resources: RwLock<alt_resource::AltResourceManager>,
}

macro_rules! with_resource {
  ($func:expr, $property_name:ident, $borrow_func:ident) => {
    paste::paste! {{
      let resource = RESOURCE.read().unwrap();
      let resource = resource.as_ref().unwrap();
      let manager = resource.[<$property_name>].[<$borrow_func>]().unwrap_or_else(|_| {
        panic!("Failed to {} `{}`", stringify!($borrow_func), stringify!($property_name));
      });
      $func(manager, resource)
    }}
  };
}

macro_rules! impl_borrow_fn {
  ($property_name:ident, $full_path:path) => {
    paste::paste! {
      pub fn [<with_  $property_name _ref>]<F, R>(f: F) -> R
      where
        F: FnOnce(RwLockReadGuard<$full_path>, &Resource) -> R,
      {
        with_resource!(f, $property_name, read)
      }
    }
  };
}

macro_rules! impl_borrow_mut_fn {
  ($property_name:ident, $full_path:path) => {
    paste::paste! {
      pub fn [<with_  $property_name _mut>]<F, R>(f: F) -> R
      where
        F: FnOnce(RwLockWriteGuard<$full_path>, &Resource) -> R,
      {
        with_resource!(f, $property_name, write)
      }
    }
  };
}

impl Resource {
  pub fn init(resource_name: StringResourceName, module_handlers: ModuleHandlers) {
    let resource = Resource {
      name: resource_name,
      module_handlers,
      ..Default::default()
    };

    resource.alt_resources.write().unwrap().init(&resource.name);
    RESOURCE.write().unwrap().replace(resource);
  }

  pub fn with<F, R>(f: F) -> R
  where
    F: FnOnce(&Resource) -> R,
  {
    let resource = RESOURCE.read().unwrap();
    f(resource.as_ref().unwrap())
  }

  fn pending_base_object_destroy_or_creation(&self) -> bool {
    self
      .pending_base_object_destroy_or_creation
      .try_write()
      .is_err()
  }

  #[allow(clippy::not_unsafe_ptr_arg_deref)]
  pub fn on_base_object_create(
    &self,
    ptr: altv_sdk::BaseObjectMutPtr,
    base_object_type: altv_sdk::BaseObjectType,
  ) {
    logger::debug!("on_base_object_create {ptr:?} {base_object_type:?}");

    if self.pending_base_object_destroy_or_creation() {
      logger::debug!("pending_base_object_destroy_or_creation -> skip");
      return;
    }

    self
      .base_objects
      .write()
      .unwrap()
      .on_create(ptr, base_object_type);
  }

  pub fn on_base_object_destroy(
    &self,
    ptr: altv_sdk::BaseObjectMutPtr,
    base_object_type: altv_sdk::BaseObjectType,
  ) {
    logger::debug!("on_base_object_destroy {ptr:?} {base_object_type:?}");

    if self.pending_base_object_destroy_or_creation() {
      logger::debug!("pending_base_object_destroy_or_creation -> skip");
      return;
    }

    self
      .base_objects
      .write()
      .unwrap()
      .on_remove(ptr, base_object_type);
  }

  impl_borrow_mut_fn!(timers, timers::TimerManager);
  impl_borrow_mut_fn!(timer_schedule, timers::ScheduleState);
  impl_borrow_mut_fn!(events, events::EventManager);
  impl_borrow_mut_fn!(local_script_events, script_events::LocalEventManager);
  impl_borrow_mut_fn!(
    local_script_events_schedule,
    script_events::LocalEventSchedule
  );
  impl_borrow_mut_fn!(client_script_events, script_events::ClientEventManager);
  impl_borrow_mut_fn!(
    client_script_events_schedule,
    script_events::ClientEventSchedule
  );
  impl_borrow_mut_fn!(base_objects, base_objects::Store);
  impl_borrow_fn!(base_objects, base_objects::Store);
  impl_borrow_mut_fn!(
    pending_base_object_destroy_or_creation,
    base_objects::PendingDestroyOrCreation
  );
  impl_borrow_fn!(alt_resources, alt_resource::AltResourceManager);
  impl_borrow_mut_fn!(alt_resources, alt_resource::AltResourceManager);
}
