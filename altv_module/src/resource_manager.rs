use resource_impl::resource_impl::ResourceImpl;
use std::{path::PathBuf, sync::Mutex};

pub type ResourceImplMutex = &'static Mutex<ResourceImpl>;

#[derive(Debug)]
pub struct ResourceManager {
    resources: Vec<ResourceImplMutex>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager { resources: vec![] }
    }

    pub fn resources_iter<'a>(&'a self) -> std::slice::Iter<'a, &'a Mutex<ResourceImpl>> {
        self.resources.iter()
    }

    pub fn add(&mut self, path: PathBuf, resource: ResourceImplMutex) {
        self.resources.push(resource);
    }
}
