use resource_impl::resource_impl::ResourceImplRef;
use std::{
    cell::RefCell,
    collections::{hash_map, HashMap},
};

thread_local! {
    pub static RESOURCE_MANAGER_INSTANCE: RefCell<ResourceManager> = RefCell::new(ResourceManager::new());
}

#[derive(Debug)]
pub struct ResourceController {
    lib: libloading::Library,
    pub resource_impl: ResourceImplRef,
}

impl ResourceController {
    pub fn new(lib: libloading::Library, resource_impl: ResourceImplRef) -> Self {
        Self { lib, resource_impl }
    }
}

impl Drop for ResourceController {
    fn drop(&mut self) {
        // TEST
        resource_impl::log_warn!("ResourceController drop lib: {:?}", self.lib);
    }
}

#[derive(Debug)]
pub struct ResourceManager {
    resources: HashMap<String, ResourceController>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: HashMap::new(),
        }
    }
    pub fn resources_iter(&self) -> hash_map::Iter<String, ResourceController> {
        self.resources.iter()
    }

    pub fn add(&mut self, full_main_path: String, resource: ResourceController) {
        self.resources.insert(full_main_path, resource);
    }

    pub fn remove(&mut self, full_main_path: &str) {
        if let Some(controller) = self.resources.remove(full_main_path) {
            controller.resource_impl.borrow().__on_remove();
        } else {
            resource_impl::log_error!("ResourceManager remove unknown resource: {full_main_path}");
        }
    }

    pub fn get_by_path(&self, full_main_path: &str) -> Option<&ResourceController> {
        self.resources.get(full_main_path)
    }
}
