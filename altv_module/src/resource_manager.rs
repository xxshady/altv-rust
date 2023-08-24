use std::{
    cell::RefCell,
    collections::{hash_map, HashMap, HashSet},
};
use altv_sdk::ffi as sdk;
use crate::ResourceName;
use crate::wasi::Exports;

thread_local! {
    pub static RESOURCE_MANAGER_INSTANCE: RefCell<ResourceManager> = RefCell::new(ResourceManager::default());
}

#[derive(Debug)]
pub struct ResourceController {
    pub ptr: *mut sdk::shared::AltResource,
    pub wasi_exports: RefCell<Exports>,
}

impl ResourceController {
    pub fn new(ptr: *mut sdk::shared::AltResource, wasi_exports: Exports) -> Self {
        Self {
            ptr,
            wasi_exports: RefCell::new(wasi_exports),
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

    pub fn get_by_name(&self, name: &str) -> Option<&ResourceController> {
        self.resources.get(name)
    }

    pub fn add_pending_status(&mut self, name: ResourceName) {
        self.pending_start_resources.insert(name);
    }

    pub fn remove_pending_status(&mut self, name: &str) {
        self.pending_start_resources.remove(name);
    }

    pub fn add(&mut self, name: ResourceName, resource: ResourceController) {
        self.resources.insert(name, resource);
    }

    pub fn remove(&mut self, resource: &str) {
        if self.resources.remove(resource).is_none() {
            logger::error!("ResourceManager remove unknown resource: {resource}");
        }
    }
}
