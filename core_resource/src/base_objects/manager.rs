use crate::{sdk, VoidResult};
use std::{collections::HashMap, fmt::Debug, ptr::NonNull};

use super::{base_impl::BaseObjectContainer, BaseObjectId};

pub(crate) struct BaseObjectManager<T, InheritPtrs: Clone = ()> {
    objects: HashMap<NonNull<T>, BaseObjectContainer<T, InheritPtrs>>,
    objects_by_id: HashMap<BaseObjectId, BaseObjectContainer<T, InheritPtrs>>,
}

impl<T, InheritPtrs: Clone> BaseObjectManager<T, InheritPtrs> {
    pub fn add(
        &mut self,
        base_ptr: altv_sdk::BaseObjectMutPtr,
        ptr: NonNull<T>,
        base_object: BaseObjectContainer<T, InheritPtrs>,
    ) {
        self.objects.insert(ptr, base_object.clone());

        let id = unsafe { sdk::IBaseObject::GetID(base_ptr.as_ptr()) };
        self.objects_by_id.insert(id, base_object);
    }

    pub fn remove(&mut self, base_ptr: altv_sdk::BaseObjectMutPtr, ptr: NonNull<T>) -> VoidResult {
        logger::debug!("remove ptr: {ptr:?}");
        if self.objects.remove(&ptr).is_some() {
            self.remove_id(base_ptr);
            Ok(())
        } else {
            anyhow::bail!("unknown base object")
        }
    }

    pub fn remove_externally(
        &mut self,
        base_ptr: altv_sdk::BaseObjectMutPtr,
        ptr: NonNull<T>,
    ) -> VoidResult {
        if let Some(obj) = self.objects.remove(&ptr) {
            obj.value.try_borrow_mut()?.clear_pointers();
            self.remove_id(base_ptr);
            Ok(())
        } else {
            anyhow::bail!("unknown base object")
        }
    }

    pub fn get_by_ptr(&self, ptr: NonNull<T>) -> Option<BaseObjectContainer<T, InheritPtrs>> {
        self.objects.get(&ptr).cloned()
    }

    pub fn has(&self, ptr: NonNull<T>) -> bool {
        self.objects.contains_key(&ptr)
    }

    pub fn all(&self) -> Vec<BaseObjectContainer<T, InheritPtrs>> {
        self.objects.values().cloned().collect()
    }

    pub fn all_count(&self) -> usize {
        self.objects.values().count()
    }

    pub fn get_by_id(&self, id: BaseObjectId) -> Option<BaseObjectContainer<T, InheritPtrs>> {
        self.objects_by_id.get(&id).cloned()
    }

    fn remove_id(&mut self, base_ptr: altv_sdk::BaseObjectMutPtr) {
        let id = unsafe { sdk::IBaseObject::GetID(base_ptr.as_ptr()) };
        self.objects_by_id.remove(&id);
    }
}

impl<T, InheritPtrs: Clone> Default for BaseObjectManager<T, InheritPtrs> {
    fn default() -> Self {
        Self {
            objects: Default::default(),
            objects_by_id: Default::default(),
        }
    }
}

impl<T, InheritPtrs: Clone> Debug for BaseObjectManager<T, InheritPtrs> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BaseObjectManager<T>")
    }
}
