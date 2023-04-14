use std::{collections::HashMap, ptr::NonNull, rc::Rc};

use core_shared::ResourceName;

use crate::{helpers::IntoString, resource::Resource, sdk};

#[derive(Debug)]
pub struct AltResource {
    pub name: String,
    pub resource_type: String,
    pub main: String,
    pub client_type: String,
    pub client_main: String,
    pub path: String,
    pub client_files: Vec<String>,
}

impl AltResource {
    pub fn all() -> Vec<Rc<AltResource>> {
        Resource::with_alt_resources_ref(|v, _| v.resources.values().cloned().collect())
    }

    pub fn current() -> Rc<AltResource> {
        Resource::with_alt_resources_ref(|v, _| v.this_resource.as_ref().unwrap().clone())
    }

    pub fn get_by_name(name: &str) -> Option<Rc<AltResource>> {
        Resource::with_alt_resources_ref(|v, _| v.get_by_name(name))
    }

    pub fn start(name: impl IntoString) {
        let raw_ptr = unsafe { sdk::ICore::StartResource(name.into_string()) };
        Resource::with_alt_resources_mut(|mut v, _| v.add_resource_from_raw_ptr(raw_ptr));
    }

    pub fn restart(&self) {
        unsafe { sdk::ICore::RestartResource(&self.name) }
    }

    pub fn stop(&self) {
        unsafe { sdk::ICore::StopResource(&self.name) }
    }
}

type ResourcePtr = NonNull<sdk::alt::IResource>;

#[derive(Debug, Default)]
pub struct AltResourceManager {
    resources: HashMap<String, Rc<AltResource>>,
    this_resource: Option<Rc<AltResource>>,
}

impl AltResourceManager {
    pub fn init(&mut self, this_name: &ResourceName) {
        let resources = unsafe { sdk::ICore::GetAllResources() };
        for v in resources.into_iter() {
            let raw_ptr = unsafe { sdk::read_resource_ptr_wrapper(v) };
            let resource = self.add_resource_from_raw_ptr(raw_ptr);
            if &resource.name == this_name {
                self.this_resource.replace(resource);
            }
        }

        if self.this_resource.is_none() {
            panic!("this resource is none");
        }
    }

    pub fn add_resource_from_raw_ptr(
        &mut self,
        raw_ptr: *mut sdk::alt::IResource,
    ) -> Rc<AltResource> {
        let ptr = NonNull::new(raw_ptr).unwrap();
        let name = get_resource_name(ptr);
        logger::debug!("adding resource: {name}");
        self.add_resource(name, ptr)
    }

    pub fn on_start(&mut self, resource_ptr: ResourcePtr) -> Rc<AltResource> {
        let name = get_resource_name(resource_ptr);
        logger::debug!("on start name: {name}");
        if let Some(resource) = self.resources.get(&name) {
            logger::debug!("resource already added");
            return resource.clone();
        }
        self.add_resource(name, resource_ptr)
    }

    pub fn on_stop(&mut self, resource_ptr: ResourcePtr) -> Rc<AltResource> {
        let name = get_resource_name(resource_ptr);
        logger::debug!("on stop name: {name}");

        self.resources.remove(&name).unwrap()
    }

    pub fn add_resource(&mut self, name: ResourceName, ptr: ResourcePtr) -> Rc<AltResource> {
        use sdk::IResource::*;
        let instance = Rc::new(AltResource {
            name: name.clone(),
            resource_type: unsafe { GetType(ptr.as_ptr()) }.to_string(),
            path: unsafe { GetPath(ptr.as_ptr()) }.to_string(),
            main: unsafe { GetMain(ptr.as_ptr()) }.to_string(),
            client_type: unsafe { GetClientType(ptr.as_ptr()) }.to_string(),
            client_main: unsafe { GetClientMain(ptr.as_ptr()) }.to_string(),
            client_files: unsafe { GetClientFiles(ptr.as_ptr()) }
                .into_iter()
                .map(|v| v.to_string())
                .collect(),
        });

        self.resources.insert(name, instance.clone());
        instance
    }

    pub fn get_by_name(&self, name: &str) -> Option<Rc<AltResource>> {
        self.resources.get(name).cloned()
    }
}

fn get_resource_name(resource_ptr: ResourcePtr) -> ResourceName {
    unsafe { sdk::IResource::GetName(resource_ptr.as_ptr()) }.to_string()
}
