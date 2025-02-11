use std::{
  cell::RefCell,
  collections::{hash_map, HashMap, HashSet},
};

use altv_sdk::ffi as sdk;
use core_shared::{ResourceName, ResourceNameRef};
use crate::{
  gen_exports::{self, ModuleExports},
  gen_imports, Module, ALTV_MODULE_VERSION,
};

thread_local! {
  pub static RESOURCE_MANAGER_INSTANCE: RefCell<ResourceManager> = RefCell::new(ResourceManager::default());
}

#[derive(Debug)]
pub struct ResourceController {
  module: Module,
}

impl ResourceController {
  pub fn new(module: Module) -> Self {
    Self { module }
  }

  pub fn exports(&self) -> &ModuleExports {
    self.module.exports()
  }
}

#[derive(Debug, Default)]
pub struct ResourceManager {
  resources: HashMap<ResourceName, ResourceController>,
  pending_start_resources: HashSet<ResourceName>,
}

impl ResourceManager {
  pub fn resources_iter(&self) -> hash_map::Iter<String, ResourceController> {
    self.resources.iter()
  }

  pub fn add_pending_status(&mut self, name: ResourceName) {
    self.pending_start_resources.insert(name);
  }

  pub fn remove_pending_status(&mut self, name: ResourceNameRef) {
    self.pending_start_resources.remove(name);
  }

  pub fn is_pending(&self, name: ResourceNameRef) -> bool {
    self.pending_start_resources.contains(name)
  }

  pub fn add(&mut self, name: ResourceName, resource: ResourceController) {
    self.resources.insert(name, resource);
  }

  pub fn remove(&mut self, resource_name: ResourceNameRef) {
    let Some(resource) = self.resources.remove(resource_name) else {
      logger::error!("Failed to remove unknown resource: {resource_name}");
      return;
    };

    #[cfg(feature = "reloading")]
    resource.module.unload().unwrap_or_else(|e| {
      panic!("Failed to unload resource: {resource_name}, cause:\n{e:#}");
    });

    #[cfg(not(feature = "reloading"))]
    {
      drop(resource);
      logger::warn!("Resource: {resource_name} is leaked since reloading is disabled");
    }
  }

  pub fn get_resource_exports_by_name(&self, name: ResourceNameRef) -> Option<&ModuleExports> {
    self
      .resources
      .get(name)
      .map(|resource| resource.module.exports())
  }

  pub fn start_resource(resource_name: ResourceName, full_main_path: String) -> bool {
    RESOURCE_MANAGER_INSTANCE.with(|manager| {
      manager
        .borrow_mut()
        .add_pending_status(resource_name.clone());

      dbg!();
      let module = unsafe {
        relib_host::load_module::<gen_exports::ModuleExports>(
          &full_main_path,
          gen_imports::init_imports,
        )
      };
      dbg!(module.is_ok());
      // TODO: don't panic here?
      let module = module
        .unwrap_or_else(|e| {
          let mut error_message = format!("reason: {e:#}");
          if let relib_host::LoadError::ModuleCompilationMismatch { .. } = e {
            error_message = format!(
              "note: if you are using reloading feature, \
              check if \"realoading\" feature is enabled or disabled \
              both in rust-module and altv crate\n\
              to enable it in rust-module compile it using: `cargo altvup release --force-recompile --reloading`\n\n\
              {error_message}"
            );
          }
          panic!("Failed to load resource: {resource_name} from: {full_main_path}\n{error_message}");
        });

      let core_ptr = unsafe { sdk::get_alt_core() };

      unsafe {
        module.exports()
          .init(core_ptr, resource_name.as_str().into())
          // TODO: stop resource on panic if reloading is enabled
          .unwrap();
      }

      let resource_version = unsafe {
        module.exports().altv_crate_version()
      }.unwrap_or_else(|| {
        unreachable!();
      });
      let resource_version: String = resource_version.into();

      if resource_version != ALTV_MODULE_VERSION {
        panic!(
          "\n\n\
          \x1b[31mRust module version ({}) does not match the version of the altv crate ({}) that you have installed!\n\
          Update rust-module (.dll/.so) or altv crate\
          \n\n\x1b[0m",
          ALTV_MODULE_VERSION,
          resource_version
        );
      }

      // TODO: don't panic here and stop resource?
      let ok: bool = unsafe {
        module.call_main().unwrap_or_else(|| {
          panic!("Resource: {resource_name:?} main function panicked");
        })
      };

      manager.borrow_mut().remove_pending_status(&resource_name);

      let resource_controller = ResourceController::new(module);
      manager.borrow_mut().add(resource_name.clone(), resource_controller);

      if !ok {
        // TODO: stop resource?
        logger::error!("Resource: {resource_name:?} main function returned error");

        return false;
      }

      true
    })
  }
}
