use once_cell::sync::OnceCell;
use resource_impl::resource_impl::ResourceImpl;
use std::{
    collections::{hash_map, HashMap},
    sync::{Mutex, MutexGuard},
};

pub type ResourceImplMutex = &'static Mutex<ResourceImpl>;

static RESOURCE_MANAGER_INSTANCE: OnceCell<Mutex<ResourceManager>> = OnceCell::new();

#[derive(Debug)]
pub struct ResourceManager {
    resources: HashMap<String, ResourceImplMutex>,
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

    pub fn resources_iter<'a>(&'a self) -> hash_map::Iter<String, &Mutex<ResourceImpl>> {
        self.resources.iter()
    }

    pub fn add(&mut self, full_main_path: String, resource: ResourceImplMutex) {
        self.resources.insert(full_main_path, resource);
    }

    pub fn get_by_path(&self, full_main_path: &str) -> Option<MutexGuard<ResourceImpl>> {
        let resource = self.resources.get(full_main_path);

        if let Some(resource) = resource {
            Some(resource.try_lock().expect("resource.try_lock() failed"))
        } else {
            None
        }
    }
}
