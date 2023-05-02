use crate::VoidResult;

use super::{base_object::BaseObject, base_ptr::BasePtr, wrapper::BaseObjectWrapper};

pub trait ValidBaseObject: BasePtr {
    fn valid(&self) -> bool {
        self.base_ptr().is_ok()
    }

    fn assert_valid(&self) -> VoidResult {
        if self.valid() {
            Ok(())
        } else {
            anyhow::bail!("BaseObject is invalid")
        }
    }
}

impl<T, InheritPtrs: Clone> ValidBaseObject for BaseObject<T, InheritPtrs> {}

impl<T, InheritPtrs: Clone> ValidBaseObject for BaseObjectWrapper<T, InheritPtrs> {}
