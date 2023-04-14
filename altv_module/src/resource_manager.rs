use std::{
    cell::RefCell,
    collections::{hash_map, HashMap, HashSet},
};

use core_module::{ResourceForModule, ResourceName};

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

    pub fn remove_pending_status(&mut self, name: &str) {
        self.pending_start_resources.remove(name);
    }

    pub fn is_pending(&self, name: &str) -> bool {
        self.pending_start_resources.contains(name)
    }

    pub fn add(&mut self, name: ResourceName, resource: ResourceController) {
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
        self.resources
            .get(name)
            .map(|resource| &resource.resource_for_module)
    }
}
