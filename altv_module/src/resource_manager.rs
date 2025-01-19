use std::{
  cell::RefCell,
  collections::{hash_map, HashMap, HashSet},
  ffi::CString,
};

use altv_sdk::ffi as sdk;
use core_module::{ModuleHandlers, ResourceForModule, ResourceHandlers, StringResourceName};
use libloading::Library;
use crate::{toggle_resource_event_type, ResourceMainFn, ALTV_MODULE_VERSION};

thread_local! {
    pub static RESOURCE_MANAGER_INSTANCE: RefCell<ResourceManager> = RefCell::new(ResourceManager::default());
}

#[derive(Debug)]
pub struct ResourceController {
  _lib: libloading::Library,
  pub resource_for_module: ResourceForModule,
}

impl ResourceController {
  pub fn new(lib: libloading::Library, resource_for_module: ResourceForModule) -> Self {
    Self {
      _lib: lib,
      resource_for_module,
    }
  }
}

#[derive(Debug, Default)]
pub struct ResourceManager {
  resources: HashMap<StringResourceName, ResourceController>,
  pending_start_resources: HashSet<StringResourceName>,
}

impl ResourceManager {
  pub fn resources_iter(&self) -> hash_map::Iter<String, ResourceController> {
    self.resources.iter()
  }

  pub fn add_pending_status(&mut self, name: StringResourceName) {
    self.pending_start_resources.insert(name);
  }

  pub fn remove_pending_status(&mut self, name: &str) {
    self.pending_start_resources.remove(name);
  }

  pub fn is_pending(&self, name: &str) -> bool {
    self.pending_start_resources.contains(name)
  }

  pub fn add(&mut self, name: StringResourceName, resource: ResourceController) {
    self.resources.insert(name, resource);
  }

  pub fn remove(&mut self, resource: &str) {
    if let Some(controller) = self.resources.remove(resource) {
      // workaround to fix crash due to drop_in_place of boxed closures
      // core::ptr::drop_in_place<alloc::boxed::Box<dyn$<core::ops::function::Fn<...
      drop(controller.resource_for_module);
    } else {
      logger::error!("ResourceManager remove unknown resource: {resource}");
    }
  }

  pub fn get_resource_for_module_by_name(&self, name: &str) -> Option<&ResourceForModule> {
    self
      .resources
      .get(name)
      .map(|resource| &resource.resource_for_module)
  }

  pub fn start_resource(resource_name: String, lib: Library, main_fn: ResourceMainFn) {
    RESOURCE_MANAGER_INSTANCE.with(|manager| {
      manager
        .borrow_mut()
        .add_pending_status(resource_name.clone());

      let core_ptr = unsafe { sdk::get_alt_core() };
      let module_handlers = ModuleHandlers::new(toggle_resource_event_type);
      let resource_handlers = ResourceHandlers::default();
      let mut resource_for_module = ResourceForModule::new(resource_handlers);

      println!("before main");
      let result = unsafe {
        main_fn(
          CString::new(ALTV_MODULE_VERSION).unwrap(),
          core_ptr,
          CString::new(resource_name.clone()).unwrap(),
          &mut resource_for_module.handlers,
          module_handlers,
        )
      };
      println!("after main");

      if !result.value {
        // TODO: stop resource?
        logger::error!("Resource: {resource_name:?} main function returned error");
      }

      manager.borrow_mut().remove_pending_status(&resource_name);

      let resource_controller = ResourceController::new(lib, resource_for_module);

      manager.borrow_mut().add(resource_name, resource_controller);
    });
  }
}
