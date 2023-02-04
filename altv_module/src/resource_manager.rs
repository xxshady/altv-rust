use once_cell::sync::OnceCell;
use resource_impl::resource_impl::ResourceImpl;
use std::{
    collections::{hash_map, HashMap},
    sync::{Mutex, MutexGuard},
};

static RESOURCE_MANAGER_INSTANCE: OnceCell<Mutex<ResourceManager>> = OnceCell::new();

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
            .set(Mutex::new(instance))
            .expect("RESOURCE_MANAGER_INSTANCE.set failed");
    }

    pub fn instance() -> MutexGuard<'static, Self> {
        RESOURCE_MANAGER_INSTANCE
            .get()
            .expect("RESOURCE_MANAGER_INSTANCE.get() failed")
            .try_lock()
            .expect("RESOURCE_MANAGER_INSTANCE try_lock() failed")
    }

    pub fn resources_iter_mut(&mut self) -> hash_map::IterMut<String, ResourceImpl> {
        self.resources.iter_mut()
    }

    pub fn add(&mut self, full_main_path: String, resource: ResourceImpl) {
        self.resources.insert(full_main_path, resource);
    }

    pub fn get_by_path(&mut self, full_main_path: &str) -> Option<&mut ResourceImpl> {
        let resource = self.resources.get_mut(full_main_path);

        if let Some(resource) = resource {
            Some(resource)
        } else {
            None
        }
    }
}
