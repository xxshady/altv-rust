use std::{fmt::Debug, ptr::NonNull};

use crate::{resource::Resource, sdk, SomeResult, VoidResult};

pub struct BaseObject<T, InheritPtrs: Clone> {
    pub(super) ptr: Option<NonNull<T>>,
    pub(super) base_ptr: Option<altv_sdk::BaseObjectMutPtr>,

    /// pointers of inherited classes between `ptr` and `base_ptr` <br>
    /// for example inherited classes of Vehicle would be: WorldObject, Entity
    pub(super) inherit_ptrs: Option<InheritPtrs>,
}

impl<T, InheritPtrs: Clone> BaseObject<T, InheritPtrs> {
    pub(crate) fn ptr(&self) -> SomeResult<NonNull<T>> {
        self.ptr.ok_or(anyhow::anyhow!("base object ptr is none"))
    }

    pub(crate) fn raw_ptr(&self) -> SomeResult<*mut T> {
        Ok(self.ptr()?.as_ptr())
    }

    pub(crate) fn internal_destroy(&mut self) -> VoidResult {
        let Some(base_ptr) = self.base_ptr else {
            anyhow::bail!("base_object already destroyed");
        };

        Resource::with_pending_base_object_destroy_or_creation_mut(|_, _| unsafe {
            sdk::ICore::DestroyBaseObject(base_ptr.as_ptr())
        });

        self.clear_pointers();

        Ok(())
    }

    pub(crate) fn clear_pointers(&mut self) {
        self.ptr.take();
        self.base_ptr.take();
        self.inherit_ptrs.take();
    }
}

impl<T, InheritPtrs: Clone> Debug for BaseObject<T, InheritPtrs> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BaseObject<T>")
    }
}
