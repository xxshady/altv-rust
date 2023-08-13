use std::{
    cell::RefCell,
    collections::{hash_map, HashMap, HashSet},
    io::BufRead,
};
use altv_sdk::ffi as sdk;
use crate::ResourceName;

thread_local! {
    pub static RESOURCE_MANAGER_INSTANCE: RefCell<ResourceManager> = RefCell::new(ResourceManager::default());
}

#[derive(Debug)]
pub struct ResourceController {
    pub ptr: *mut sdk::shared::AltResource,
}

impl ResourceController {
    pub fn new(ptr: *mut sdk::shared::AltResource) -> Self {
        Self { ptr }
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

    // TODO:
    // pub fn remove(&mut self, resource: &str) {
    //     if let Some(controller) = self.resources.remove(resource) {
    //     } else {
    //         logger::error!("ResourceManager remove unknown resource: {resource}");
    //     }
    // }
}
