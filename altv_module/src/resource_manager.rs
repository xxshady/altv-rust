use std::{
    cell::{RefCell, Cell},
    collections::{HashMap, HashSet},
};
use altv_sdk::ffi as sdk;
use crate::{
    ResourceName,
    wasi::{State, imports, Exports, self},
    wasi_stdout_err::{UnimplementedStdout, Stderr},
};

thread_local! {
    pub static RESOURCE_MANAGER_INSTANCE: RefCell<ResourceManager> = RefCell::new(ResourceManager::default());
    /// Pending base object creation or deletion
    pub static PENDING_BASE_OBJECT: Cell<bool> = Cell::new(false);
}

#[derive(Debug)]
pub struct ResourceController {
    pub ptr: *mut sdk::shared::AltResource,
    pub name: ResourceName,

    exports: RefCell<Exports>,
    current_error_message: RefCell<Option<Vec<u8>>>,
}

impl ResourceController {
    pub fn new(
        name: ResourceName,
        ptr: *mut sdk::shared::AltResource,
        wasm_bytes: &[u8],
    ) -> Result<ResourceController, wasmtime::Error> {
        let exports = Self::start_wasi(name.clone(), wasm_bytes, ptr)?;
        Ok(Self {
            ptr,
            exports: RefCell::new(exports),
            current_error_message: RefCell::new(None),
            name,
        })
    }

    fn start_wasi(
        resource_name: String,
        wasm_bytes: &[u8],
        resource_ptr: *mut sdk::shared::AltResource,
    ) -> wasmtime::Result<Exports> {
        use wasmtime::*;
        use wasmtime_wasi::WasiCtxBuilder;

        std::env::set_var("WASMTIME_BACKTRACE_DETAILS", "1");

        let engine = Engine::new(Config::new().cranelift_opt_level(OptLevel::Speed)).unwrap();

        let mut linker = Linker::<State>::new(&engine);

        let module = Module::from_binary(&engine, wasm_bytes)?;

        let wasi = WasiCtxBuilder::new()
            .env("RUST_BACKTRACE", "full")?
            .stdout(Box::new(UnimplementedStdout(std::io::stdout())))
            .stderr(Box::new(Stderr(std::io::stderr(), resource_name)))
            .build();

        let mut store = Store::new(&engine, State::new(wasi, resource_ptr));

        wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi)?;
        imports::add_to_linker(&mut linker);

        let instance = linker.instantiate(&mut store, &module)?;

        Ok(Exports::new(|s| &mut s.big_call_ptr, store, instance))
    }

    pub fn call_main(&self) -> wasmtime::Result<()> {
        let mut exports = self.exports.borrow_mut();

        exports.call_pre_main()?;
        exports.call_main()
    }

    pub fn call_export<T>(
        &self,
        call: impl FnOnce(&mut wasi::Exports) -> wasmtime::Result<T>,
    ) -> wasmtime::Result<T> {
        let mut exports = self.exports.borrow_mut();
        let res = call(&mut *exports);

        match res {
            Ok(v) => {
                self.ensure_error_message_is_empty();
                Ok(v)
            }
            Err(err) => {
                logger::error!(
                    "Resource: {:?} internal export call failed with error: {err:?}",
                    self.name
                );
                self.log_error_message();
                Err(err)
            }
        }
    }

    pub fn extend_error_message(&self, buf: &[u8]) {
        self.current_error_message
            .borrow_mut()
            .get_or_insert(Vec::new())
            .extend_from_slice(buf);
    }

    pub fn log_error_message(&self) {
        let Some(message) = self.current_error_message.borrow_mut().take() else {
            return;
        };

        let Ok(message) = std::str::from_utf8(&message) else {
            logger::error!(
                "Failed to create str from raw error message of resource: {}",
                self.name
            );
            return;
        };

        unsafe { altv_sdk::helpers::log_error_with_resource(message, self.ptr) }
    }

    pub fn ensure_error_message_is_empty(&self) {
        if self.current_error_message.borrow_mut().take().is_none() {
            return;
        }

        logger::error!("Resource: {} tried to output something to stderr without panic (probably called `dbg!` or something like this? use `altv::dbg!` instead)", self.name);
    }
}

#[derive(Debug, Default)]
pub struct ResourceManager {
    resources: HashMap<ResourceName, ResourceController>,
    pending_start_resources: HashSet<ResourceName>,
    panicked_resources: HashSet<ResourceName>,
}

impl ResourceManager {
    pub fn get_by_name(&self, name: &str) -> Option<&ResourceController> {
        self.resources.get(name)
    }

    pub fn is_resource_starting(&self, name: &str) -> bool {
        self.pending_start_resources.contains(name)
    }

    pub fn add_pending_status(&mut self, name: ResourceName) {
        self.pending_start_resources.insert(name);
    }

    pub fn remove_pending_status(&mut self, name: &str) {
        self.pending_start_resources.remove(name);
    }

    pub fn add(&mut self, name: ResourceName, resource: ResourceController) {
        self.resources.insert(name, resource);
    }

    pub fn remove(&mut self, resource: &str) {
        self.pending_start_resources.remove(resource);

        if self.panicked_resources.contains(resource) {
            self.panicked_resources.remove(resource);
            logger::debug!("tried to remove panicked resource: {resource}, ignoring");
            return;
        }

        if self.resources.remove(resource).is_none() {
            logger::error!("remove unknown resource: {resource}");
        }
    }

    pub fn resource_panicked(&mut self, resource: ResourceName) {
        logger::debug!("resource_panicked: {resource}");

        if self.resources.remove(&resource).is_none() {
            logger::error!("remove unknown panicked resource: {resource}");
            return;
        }

        self.panicked_resources.insert(resource);
    }
}

pub fn set_pending_base_object(val: bool) {
    PENDING_BASE_OBJECT.with(|v| {
        v.set(val);
    })
}

pub fn is_pending_base_object() -> bool {
    PENDING_BASE_OBJECT.with(|v| v.get())
}
