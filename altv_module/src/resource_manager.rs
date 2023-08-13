use std::{
    cell::RefCell,
    collections::{hash_map, HashMap, HashSet},
    io::BufRead,
};
use altv_sdk::ffi as sdk;
use crate::{ResourceName, types::StdoutReader};

thread_local! {
    pub static RESOURCE_MANAGER_INSTANCE: RefCell<ResourceManager> = RefCell::new(ResourceManager::default());
}

#[derive(Debug)]
pub struct ResourceController {
    instance: wasmer::Instance,
    stdout: RefCell<StdoutReader>,
    store: RefCell<wasmer::Store>,
    // exports: Exports,
    pub ptr: *mut sdk::shared::AltResource,
}

impl ResourceController {
    pub fn new(
        instance: wasmer::Instance,
        stdout: StdoutReader,
        ptr: *mut sdk::shared::AltResource,
        // exports: Exports,
        store: wasmer::Store,
    ) -> Self {
        Self {
            instance,
            stdout: RefCell::new(stdout),
            store: RefCell::new(store),
            ptr,
            // exports,
        }
    }

    pub fn read_stdout_line(&self) -> String {
        let mut stdout_buf = String::new();

        let stdout = self.stdout.try_borrow_mut();
        let Ok(mut stdout) = stdout else {
            logger::error!("Failed to mut borrow stdout");
            return stdout_buf;
        };

        let _ = stdout.read_line(&mut stdout_buf);
        stdout_buf
    }

    pub fn call_exports(
        &self,
        // f: impl Fn(&Exports, &mut wasmer::Store) -> Result<(), wasmer::RuntimeError>,
    ) {
        // let store = self.store.try_borrow_mut();
        // let Ok(mut store) = store else {
        //     logger::error!("Failed to mut borrow store");
        //     return;
        // };

        // let res = f(&self.exports, &mut store);
        // if let Err(e) = res {
        //     logger::error!("Failed to call exports, error: {e:?}");
        // }
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
