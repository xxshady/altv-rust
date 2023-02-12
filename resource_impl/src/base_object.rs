use std::{
    any::Any,
    collections::HashMap,
    fmt::Debug,
    ops::Deref,
    rc::Rc,
    sync::{Arc, Mutex},
};

use altv_sdk::ffi as sdk;
use once_cell::sync::OnceCell;

pub(crate) type RawBaseObjectPointer = *mut sdk::IBaseObject;

macro_rules! convert_ptr_to {
    ($self: ident, $sdk_converter: path) => {
        if let Some(raw) = $self.0 {
            Ok(unsafe { $sdk_converter(raw) })
        } else {
            Err("Raw base object pointer is none".to_string())
        }
    };
}

#[derive(Debug)]
pub struct BaseObjectPointer(Option<RawBaseObjectPointer>);

impl BaseObjectPointer {
    pub fn new(raw_ptr: RawBaseObjectPointer) -> Self {
        Self(Some(raw_ptr))
    }

    pub fn get(&self) -> Result<RawBaseObjectPointer, String> {
        if let Some(raw) = self.0 {
            Ok(raw)
        } else {
            Err("Raw base object pointer is none".to_string())
        }
    }

    pub fn set(&mut self, value: Option<RawBaseObjectPointer>) {
        self.0 = value;
    }

    pub fn to_entity(&self) -> Result<*mut sdk::IEntity, String> {
        convert_ptr_to!(self, sdk::convert_baseobject_to_entity)
    }

    pub fn to_vehicle(&self) -> Result<*mut sdk::IVehicle, String> {
        convert_ptr_to!(self, sdk::convert_baseobject_to_vehicle)
    }

    pub fn to_player(&self) -> Result<*mut sdk::IPlayer, String> {
        convert_ptr_to!(self, sdk::convert_baseobject_to_player)
    }
}

// TEST
unsafe impl Send for BaseObjectPointer {}
unsafe impl Sync for BaseObjectPointer {}

// TEST
// pub(crate) trait BaseObject {
pub trait BaseObject {
    fn as_any(&self) -> &dyn Any;
    fn ptr(&self) -> &BaseObjectPointer;
    fn ptr_mut(&mut self) -> &mut BaseObjectPointer;
    fn base_type(&self) -> altv_sdk::BaseObjectType;

    fn destroy_base_object(&mut self) -> Result<(), String> {
        if let Ok(raw_ptr) = self.ptr().get() {
            let _deletion = BASE_OBJECT_DELETION_INSTANCE
                .get()
                .unwrap()
                .try_lock()
                .unwrap();

            unsafe { sdk::destroy_baseobject(raw_ptr) }
            self.ptr_mut().set(None);

            BASE_OBJECT_MANAGER_INSTANCE
                .get()
                .unwrap()
                .try_lock()
                .unwrap()
                .remove(raw_ptr);

            Ok(())
        } else {
            Err("Base object is already destroyed".to_string())
        }
    }
}

#[derive(Debug)]
struct Blocker<T: ?Sized>(T);

impl<T> Deref for Blocker<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// TEST
// pub(crate) type BaseObjectContainer = Arc<Mutex<dyn BaseObject + Send + Sync>>;
pub(crate) type BaseObjectContainer = Arc<Mutex<dyn BaseObject + Send + Sync>>;

pub(crate) static BASE_OBJECT_MANAGER_INSTANCE: OnceCell<Mutex<BaseObjectManager>> =
    OnceCell::new();

impl Debug for dyn BaseObject + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dyn BaseObject + Send + Sync")
    }
}

#[derive(Debug)]
pub struct BaseObjectManager {
    // usize is RawBaseObjectPointer
    base_objects: HashMap<usize, BaseObjectContainer>,
    // concrete: HashMap<usize, BaseObjectContainer>,
}

impl BaseObjectManager {
    pub fn new() -> Self {
        Self {
            base_objects: HashMap::new(),
        }
    }

    pub fn on_create(&mut self, raw_ptr: RawBaseObjectPointer, base_object: BaseObjectContainer) {
        self.base_objects.insert(raw_ptr as usize, base_object);
    }

    pub fn on_destroy(&mut self, base_object: BaseObjectContainer) {
        let mut base_object = base_object.try_lock().unwrap();
        let raw_ptr = base_object.ptr().get().unwrap();
        base_object.ptr_mut().set(None);

        if self.base_objects.remove(&(raw_ptr as usize)).is_some() {
            crate::log!("~gl~BaseObjectManager destroyed object: {raw_ptr:?}");
        } else {
            crate::log_error!("BaseObjectManager on_destroy invalid object: {raw_ptr:?}");
        }
    }

    fn remove(&mut self, raw_ptr: RawBaseObjectPointer) {
        if self.base_objects.remove(&(raw_ptr as usize)).is_some() {
            crate::log!("~gl~BaseObjectManager removed object: {raw_ptr:?}");
        } else {
            crate::log_error!("BaseObjectManager remove invalid object: {raw_ptr:?}");
        }
    }

    pub fn get_by_raw_ptr(&self, raw_ptr: RawBaseObjectPointer) -> Option<BaseObjectContainer> {
        self.base_objects.get(&(raw_ptr as usize)).cloned()
    }
}

pub(crate) static BASE_OBJECT_CREATION_INSTANCE: OnceCell<Mutex<PendingBaseObjectCreation>> =
    OnceCell::new();
pub(crate) static BASE_OBJECT_DELETION_INSTANCE: OnceCell<Mutex<PendingBaseObjectDeletion>> =
    OnceCell::new();

#[derive(Debug)]
pub struct PendingBaseObjectCreation;

impl PendingBaseObjectCreation {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub(crate) struct PendingBaseObjectDeletion;

impl PendingBaseObjectDeletion {
    pub fn new() -> Self {
        Self
    }
}
