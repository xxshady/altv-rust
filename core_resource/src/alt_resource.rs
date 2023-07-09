use std::{collections::HashMap, ptr::NonNull, rc::Rc, cell::RefCell};

use anyhow::bail;
use core_shared::ResourceName;

use crate::{
    config_node::ResourceConfig,
    helpers::{read_cpp_str_vec, IntoString},
    resource::Resource,
    sdk, SomeResult, VoidResult,
};

#[derive(Debug)]
pub struct AltResource {
    pub name: String,
    pub resource_type: String,
    pub main: String,
    pub client_type: String,
    pub client_main: String,
    pub path: String,
    pub client_files: Vec<String>,
    pub dependants: Vec<String>,
    pub dependencies: Vec<String>,
    pub config: ResourceConfig,

    valid: RefCell<bool>,
}

impl AltResource {
    pub fn all() -> Vec<Rc<AltResource>> {
        Resource::with_alt_resources_ref(|v, _| v.resources.values().cloned().collect())
    }

    pub fn current() -> Rc<AltResource> {
        Resource::with_alt_resources_ref(|v, _| v.this_resource.as_ref().unwrap().clone())
    }

    pub fn get_by_name(name: &str) -> SomeResult<Rc<AltResource>> {
        Resource::with_alt_resources_ref(|v, _| v.get_by_name(name))
    }

    pub fn start(name: impl IntoString) {
        let raw_ptr = unsafe { sdk::ICore::StartResource(name.into_string()) };
        Resource::with_alt_resources_mut(|mut v, _| v.add_resource_from_raw_ptr(raw_ptr));
    }

    pub fn restart(&self) -> VoidResult {
        self.assert_valid()?;
        unsafe { sdk::ICore::RestartResource(&self.name) }
        Ok(())
    }

    pub fn stop(&self) -> VoidResult {
        self.assert_valid()?;
        unsafe { sdk::ICore::StopResource(&self.name) }
        Ok(())
    }

    pub fn valid(&self) -> SomeResult<bool> {
        Ok(*self.valid.try_borrow()?)
    }

    fn assert_valid(&self) -> VoidResult {
        if !self.valid()? {
            bail!(
                "Resource: '{}' is stopped and cannot be used anymore",
                self.name
            )
        }
        Ok(())
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

        let resource = self.resources.remove(&name).unwrap();

        let Ok(mut valid) = resource.valid.try_borrow_mut() else {
            logger::error!("Failed to invalidate resource: '{name}' on stop");
            return resource;
        };

        *valid = false;
        drop(valid);

        resource
    }

    pub fn add_resource(&mut self, name: ResourceName, ptr: ResourcePtr) -> Rc<AltResource> {
        use sdk::IResource::*;

        let raw_ptr = ptr.as_ptr();
        let instance = Rc::new(AltResource {
            name: name.clone(),
            resource_type: unsafe { GetType(raw_ptr) }.to_string(),
            path: unsafe { GetPath(raw_ptr) }.to_string(),
            main: unsafe { GetMain(raw_ptr) }.to_string(),
            client_type: unsafe { GetClientType(raw_ptr) }.to_string(),
            client_main: unsafe { GetClientMain(raw_ptr) }.to_string(),
            client_files: read_cpp_str_vec(unsafe { GetClientFiles(raw_ptr) }),
            dependants: read_cpp_str_vec(unsafe { GetDependants(raw_ptr) }),
            dependencies: read_cpp_str_vec(unsafe { GetDependencies(raw_ptr) }),
            config: ResourceConfig::new(unsafe { GetConfig(raw_ptr) }),

            valid: RefCell::new(true),
        });

        self.resources.insert(name, instance.clone());
        instance
    }

    pub fn get_by_name(&self, name: &str) -> SomeResult<Rc<AltResource>> {
        self.resources
            .get(name)
            .cloned()
            .ok_or(anyhow::anyhow!("unknown resource name: {name}"))
    }
}

fn get_resource_name(resource_ptr: ResourcePtr) -> ResourceName {
    unsafe { sdk::IResource::GetName(resource_ptr.as_ptr()) }.to_string()
}
