use resource_impl::resource_impl::ResourceImplContainer;
use std::{
    cell::RefCell,
    collections::{hash_map, HashMap},
};

thread_local! {
    pub static RESOURCE_MANAGER_INSTANCE: RefCell<ResourceManager> = RefCell::new(ResourceManager::new());
}

#[derive(Debug)]
pub struct ResourceManager {
    resources: HashMap<String, ResourceImplContainer>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: HashMap::new(),
        }
    }
    pub fn resources_iter(&self) -> hash_map::Iter<String, ResourceImplContainer> {
        self.resources.iter()
    }

    pub fn add(&mut self, full_main_path: String, resource: ResourceImplContainer) {
        self.resources.insert(full_main_path, resource);
    }

    pub fn get_by_path(&self, full_main_path: &str) -> Option<&ResourceImplContainer> {
        let resource = self.resources.get(full_main_path);

        if let Some(resource) = resource {
            Some(resource)
        } else {
            None
        }
    }
}
