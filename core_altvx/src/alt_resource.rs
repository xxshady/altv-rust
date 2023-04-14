use std::{collections::HashMap, ptr::NonNull, rc::Rc};

use crate::{helpers::IntoString, sdk};

#[derive(Debug)]
pub struct AltResource {
    pub name: String,
}

impl AltResource {
    pub fn start(name: impl IntoString) {
        unsafe { sdk::ICore::StartResource(name.into_string()) };
    }

    pub fn restart(name: impl IntoString) {
        unsafe { sdk::ICore::RestartResource(name.into_string()) }
    }

    pub fn stop(name: impl IntoString) {
        unsafe { sdk::ICore::StopResource(name.into_string()) }
    }
}

type ResourcePtr = NonNull<sdk::alt::IResource>;

#[derive(Debug, Default)]
pub struct AltResourceManager {
    resources: HashMap<String, Rc<AltResource>>,
}

impl AltResourceManager {
    pub fn on_start(&mut self, resource_ptr: ResourcePtr) -> Rc<AltResource> {
        let name = unsafe { sdk::IResource::GetName(resource_ptr.as_ptr()) }.to_string();
        logger::debug!("on start name: {name}");

        let instance = Rc::new(AltResource { name: name.clone() });
        self.resources.insert(name, instance.clone());
        instance
    }

    pub fn on_stop(&mut self, resource_ptr: ResourcePtr) -> Rc<AltResource> {
        let name = unsafe { sdk::IResource::GetName(resource_ptr.as_ptr()) }.to_string();
        logger::debug!("on stop name: {name}");

        self.resources.remove(&name).unwrap()
    }
}
