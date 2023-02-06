use once_cell::sync::OnceCell;
use resource_impl::resource_impl::ResourceImpl;
use std::{
    collections::{hash_map, HashMap},
    sync::{RwLock, RwLockReadGuard, RwLockWriteGuard},
};

// TODO: since we dont need multi-threading here its better to migrate to thread_local refcell
static RESOURCE_MANAGER_INSTANCE: OnceCell<RwLock<ResourceManager>> = OnceCell::new();

#[derive(Debug)]
pub struct ResourceManager {
    resources: HashMap<String, ResourceImpl>,
}

impl ResourceManager {
    pub fn init() {
        let instance = ResourceManager {
            resources: HashMap::new(),
        };

        RESOURCE_MANAGER_INSTANCE
            .set(RwLock::new(instance))
            .expect("RESOURCE_MANAGER_INSTANCE.set failed");
    }

    pub fn instance() -> RwLockReadGuard<'static, Self> {
        RESOURCE_MANAGER_INSTANCE
            .get()
            .expect("RESOURCE_MANAGER_INSTANCE.get() failed")
            .try_read()
            .expect("RESOURCE_MANAGER_INSTANCE try_read() failed")
    }

    pub fn instance_mut() -> RwLockWriteGuard<'static, Self> {
        RESOURCE_MANAGER_INSTANCE
            .get()
            .expect("RESOURCE_MANAGER_INSTANCE.get() failed")
            .try_write()
            .expect("RESOURCE_MANAGER_INSTANCE try_write() failed")
    }

    pub fn resources_iter_mut(&self) -> hash_map::Iter<String, ResourceImpl> {
        self.resources.iter()
    }

    pub fn add(&mut self, full_main_path: String, resource: ResourceImpl) {
        self.resources.insert(full_main_path, resource);
    }

    pub fn get_by_path(&self, full_main_path: &str) -> Option<&ResourceImpl> {
        let resource = self.resources.get(full_main_path);

        if let Some(resource) = resource {
            Some(resource)
        } else {
            None
        }
    }
}
