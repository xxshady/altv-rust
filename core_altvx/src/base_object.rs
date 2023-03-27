use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::Rc};

use altv_sdk::{ffi as sdk, IBaseObjectMutPtr};

use crate::resource::Resource;

macro_rules! convert_ptr_to {
    ($self: ident, $sdk_converter: path) => {
        if let Some(raw) = $self.0 {
            Ok(unsafe { $sdk_converter(raw) })
        } else {
            $crate::anyhow::bail!(
                "[{}] Raw base object pointer is none",
                stringify!($sdk_converter)
            )
        }
    };
}

#[derive(Debug)]
pub struct BaseObjectPointer(Option<IBaseObjectMutPtr>);

impl BaseObjectPointer {
    pub fn new(raw_ptr: IBaseObjectMutPtr) -> Self {
        Self(Some(raw_ptr))
    }

    pub fn valid(&self) -> bool {
        self.0.is_some()
    }

    pub fn get(&self) -> Result<IBaseObjectMutPtr, String> {
        if let Some(raw) = self.0 {
            Ok(raw)
        } else {
            Err("Raw base object pointer is none".to_string())
        }
    }

    pub fn set(&mut self, value: Option<IBaseObjectMutPtr>) {
        self.0 = value;
    }

    pub fn to_world_object(&self) -> anyhow::Result<*mut sdk::alt::IWorldObject> {
        convert_ptr_to!(self, sdk::base_object::to_world_object)
    }

    pub fn to_entity(&self) -> anyhow::Result<*mut sdk::alt::IEntity> {
        convert_ptr_to!(self, sdk::base_object::to_entity)
    }

    pub fn to_vehicle(&self) -> anyhow::Result<*mut sdk::alt::IVehicle> {
        convert_ptr_to!(self, sdk::base_object::to_vehicle)
    }

    pub fn to_player(&self) -> anyhow::Result<*mut sdk::alt::IPlayer> {
        convert_ptr_to!(self, sdk::base_object::to_player)
    }

    pub fn to_virtual_entity(&self) -> anyhow::Result<*mut sdk::alt::IVirtualEntity> {
        convert_ptr_to!(self, sdk::base_object::to_virtual_entity)
    }

    pub fn to_virtual_entity_group(&self) -> anyhow::Result<*mut sdk::alt::IVirtualEntityGroup> {
        convert_ptr_to!(self, sdk::base_object::to_virtual_entity_group)
    }
}

pub trait BaseObject {
    fn valid(&self) -> bool;
    fn ptr(&self) -> &BaseObjectPointer;
    fn ptr_mut(&mut self) -> &mut BaseObjectPointer;
    fn base_type(&self) -> altv_sdk::BaseObjectType;

    fn destroy_base_object(&mut self) -> anyhow::Result<()> {
        if let Ok(raw_ptr) = self.ptr().get() {
            Resource::with_base_object_deletion_mut(|_, resource| {
                unsafe { sdk::ICore::DestroyBaseObject(raw_ptr) }
                self.ptr_mut().set(None);

                resource.base_objects.borrow_mut().remove(raw_ptr);

                Ok(())
            })
        } else {
            anyhow::bail!("Base object is already destroyed")
        }
    }
}

#[macro_export]
macro_rules! impl_base_object_for {
    ($struct: path) => {
        impl BaseObject for $struct {
            fn valid(&self) -> bool {
                self.ptr.get().is_ok()
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

pub type BaseObjectContainer = Rc<RefCell<dyn BaseObject>>;

impl Debug for dyn BaseObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dyn BaseObject")
    }
}

#[derive(Debug, Default)]
pub struct BaseObjectManager {
    base_objects: HashMap<IBaseObjectMutPtr, BaseObjectContainer>,
}

impl BaseObjectManager {
    pub fn on_create(&mut self, raw_ptr: IBaseObjectMutPtr, base_object: BaseObjectContainer) {
        self.base_objects.insert(raw_ptr, base_object);
    }

    pub fn on_destroy(&mut self, base_object: BaseObjectContainer) {
        let mut base_object = base_object.borrow_mut();
        let raw_ptr = base_object.ptr().get().unwrap();
        base_object.ptr_mut().set(None);

        if self.base_objects.remove(&raw_ptr).is_some() {
            logger::debug!("~gl~BaseObjectManager destroyed object: {raw_ptr:?}");
        } else {
            logger::error!("BaseObjectManager on_destroy invalid object: {raw_ptr:?}");
        }
    }

    fn remove(&mut self, raw_ptr: IBaseObjectMutPtr) {
        if self.base_objects.remove(&raw_ptr).is_some() {
            logger::debug!("~gl~BaseObjectManager removed object: {raw_ptr:?}");
        } else {
            logger::error!("BaseObjectManager remove invalid object: {raw_ptr:?}");
        }
    }

    pub fn get_by_raw_ptr(&self, raw_ptr: IBaseObjectMutPtr) -> Option<BaseObjectContainer> {
        self.base_objects.get(&raw_ptr).cloned()
    }
}

#[derive(Debug, Default)]
pub struct PendingBaseObjectCreation;

#[derive(Debug, Default)]
pub struct PendingBaseObjectDeletion;
