use crate::SomeResult;

use super::{base_object::BaseObject, wrapper::BaseObjectWrapper};

pub trait BasePtr {
    fn base_ptr(&self) -> SomeResult<altv_sdk::BaseObjectMutPtr>;
    fn raw_base_ptr(&self) -> SomeResult<altv_sdk::BaseObjectRawMutPtr>;
}

impl<T, InheritPtrs: Clone> BasePtr for BaseObject<T, InheritPtrs> {
    fn base_ptr(&self) -> SomeResult<altv_sdk::BaseObjectMutPtr> {
        self.base_ptr
            .ok_or(anyhow::anyhow!("base object base_ptr is none"))
    }

    fn raw_base_ptr(&self) -> SomeResult<altv_sdk::BaseObjectRawMutPtr> {
        Ok(self.base_ptr()?.as_ptr())
    }
}

impl<T, InheritPtrs: Clone> BasePtr for BaseObjectWrapper<T, InheritPtrs> {
    fn base_ptr(&self) -> SomeResult<altv_sdk::BaseObjectMutPtr> {
        self.value.try_borrow()?.base_ptr()
    }

    fn raw_base_ptr(&self) -> SomeResult<altv_sdk::BaseObjectRawMutPtr> {
        self.value.try_borrow()?.raw_base_ptr()
    }
}
