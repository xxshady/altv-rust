use std::{any::Any, cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use altv_sdk::ffi as sdk;

use crate::resource_impl::with_resource_impl;

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
        convert_ptr_to!(self, sdk::convert_base_object_to_entity)
    }

    pub fn to_vehicle(&self) -> Result<*mut sdk::IVehicle, String> {
        convert_ptr_to!(self, sdk::convert_base_object_to_vehicle)
    }

    pub fn to_player(&self) -> Result<*mut sdk::IPlayer, String> {
        convert_ptr_to!(self, sdk::convert_base_object_to_player)
    }
}

// TEST
unsafe impl Send for BaseObjectPointer {}
unsafe impl Sync for BaseObjectPointer {}

pub trait BaseObject {
    fn as_any(&mut self) -> &mut dyn Any;
    fn ptr(&self) -> &BaseObjectPointer;
    fn ptr_mut(&mut self) -> &mut BaseObjectPointer;
    fn base_type(&self) -> altv_sdk::BaseObjectType;

    fn destroy_base_object(&mut self) -> Result<(), String> {
        if let Ok(raw_ptr) = self.ptr().get() {
            with_resource_impl(|instance| {
                let _deletion = instance.borrow_mut_base_object_deletion();

                unsafe { sdk::destroy_base_object(raw_ptr) }
                self.ptr_mut().set(None);

                instance.borrow_mut_base_objects().remove(raw_ptr);

                Ok(())
            })
        } else {
            Err("Base object is already destroyed".to_string())
        }
    }
}

#[macro_export]
macro_rules! impl_base_object_for {
    ($struct: path) => {
        impl BaseObject for $struct {
            fn as_any(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn ptr(&self) -> &BaseObjectPointer {
                &self.ptr
            }

            fn ptr_mut(&mut self) -> &mut BaseObjectPointer {
                &mut self.ptr
            }

            fn base_type(&self) -> altv_sdk::BaseObjectType {
                self.base_type
            }
        }
    };
}

pub(crate) type BaseObjectContainer = Rc<RefCell<dyn BaseObject>>;

impl Debug for dyn BaseObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dyn BaseObject")
    }
}

#[derive(Debug)]
pub struct BaseObjectManager {
    base_objects: HashMap<RawBaseObjectPointer, BaseObjectContainer>,
}

impl BaseObjectManager {
    pub fn new() -> Self {
        Self {
            base_objects: HashMap::new(),
        }
    }

    pub fn on_create(&mut self, raw_ptr: RawBaseObjectPointer, base_object: BaseObjectContainer) {
        self.base_objects.insert(raw_ptr, base_object);
    }

    pub fn on_destroy(&mut self, base_object: BaseObjectContainer) {
        let mut base_object = base_object.borrow_mut();
        let raw_ptr = base_object.ptr().get().unwrap();
        base_object.ptr_mut().set(None);

        if self.base_objects.remove(&raw_ptr).is_some() {
            crate::log!("~gl~BaseObjectManager destroyed object: {raw_ptr:?}");
        } else {
            crate::log_error!("BaseObjectManager on_destroy invalid object: {raw_ptr:?}");
        }
    }

    fn remove(&mut self, raw_ptr: RawBaseObjectPointer) {
        if self.base_objects.remove(&raw_ptr).is_some() {
            crate::log!("~gl~BaseObjectManager removed object: {raw_ptr:?}");
        } else {
            crate::log_error!("BaseObjectManager remove invalid object: {raw_ptr:?}");
        }
    }

    pub fn get_by_raw_ptr(&self, raw_ptr: RawBaseObjectPointer) -> Option<BaseObjectContainer> {
        self.base_objects.get(&raw_ptr).cloned()
    }
}

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
