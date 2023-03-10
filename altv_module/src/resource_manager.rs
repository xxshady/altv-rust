use std::{
    cell::RefCell,
    collections::{hash_map, HashMap, HashSet},
};

use core_module::ResourceForModule;

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

pub type ResourceFullMainPath = String;

#[derive(Debug, Default)]
pub struct ResourceManager {
    resources: HashMap<ResourceFullMainPath, ResourceController>,
    pending_start_resources: HashSet<ResourceFullMainPath>,
}

impl ResourceManager {
    pub fn resources_iter(&self) -> hash_map::Iter<String, ResourceController> {
        self.resources.iter()
    }

    pub fn add_pending_status(&mut self, full_main_path: String) {
        self.pending_start_resources.insert(full_main_path);
    }

    pub fn remove_pending_status(&mut self, full_main_path: &str) {
        self.pending_start_resources.remove(full_main_path);
    }

    pub fn is_pending(&self, full_main_path: &str) -> bool {
        self.pending_start_resources.contains(full_main_path)
    }

    pub fn add(&mut self, full_main_path: String, resource: ResourceController) {
        self.resources.insert(full_main_path, resource);
    }

    pub fn remove(&mut self, full_main_path: &str) {
        if let Some(controller) = self.resources.remove(full_main_path) {
            // workaround to fix crash due to drop_in_place of boxed closures
            // core::ptr::drop_in_place<alloc::boxed::Box<dyn$<core::ops::function::Fn<...
            drop(controller.resource_for_module);
        } else {
            logger::error!("ResourceManager remove unknown resource: {full_main_path}");
        }
    }

    pub fn get_resource_for_module_by_path(
        &self,
        full_main_path: &str,
    ) -> Option<&ResourceForModule> {
        self.resources
            .get(full_main_path)
            .map(|resource| &resource.resource_for_module)
    }
}
