use std::{fmt::Debug, ptr::NonNull};

use crate::{base_objects::BasePtr, sdk, SomeResult, VoidResult};

pub struct BaseObject<T, InheritPtrs: Clone> {
    pub(super) ptr: Option<NonNull<T>>,
    pub(super) base_ptr: Option<altv_sdk::BaseObjectMutPtr>,

    /// pointers of inherited classes between `ptr` and `base_ptr` <br>
    /// for example inherited classes of Vehicle would be: WorldObject, Entity
    pub(super) inherit_ptrs: Option<InheritPtrs>,
}

impl<T, InheritPtrs: Clone> BaseObject<T, InheritPtrs> {
    pub(crate) fn ptr(&self) -> SomeResult<NonNull<T>> {
        self.ptr.ok_or(anyhow::anyhow!(
            "Base object is destroyed and cannot be used anymore (base object ptr is none)"
        ))
    }

    pub(crate) fn raw_ptr(&self) -> SomeResult<*mut T> {
        Ok(self.ptr()?.as_ptr())
    }

    pub(crate) fn internal_destroy(&mut self) -> VoidResult {
        let Some(base_ptr) = self.base_ptr else {
            anyhow::bail!("base_object already destroyed");
        };

        unsafe {
            sdk::ICore::DestroyBaseObject(base_ptr.as_ptr());
        }
        self.clear_pointers();

        Ok(())
    }

    pub(crate) fn clear_pointers(&mut self) {
        self.ptr.take();
        self.base_ptr.take();
        self.inherit_ptrs.take();
    }

    pub fn id(&self) -> SomeResult<u32> {
        Ok(unsafe { sdk::IBaseObject::GetID(self.base_ptr()?.as_ptr()) })
    }
}

impl<T, InheritPtrs: Clone> Debug for BaseObject<T, InheritPtrs> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BaseObject<T>")
    }
}

impl<T, InheritPtrs: Clone> PartialEq for BaseObject<T, InheritPtrs> {
    fn eq(&self, other: &Self) -> bool {
        let Some(ptr) = self.ptr else {
            return false;
        };
        let Some(other_ptr) = other.ptr else {
            return false;
        };

        ptr == other_ptr
    }
}
