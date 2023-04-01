use crate::VoidResult;
use std::{collections::HashMap, hash::Hash, ptr::NonNull};

use super::base_object::BaseObjectContainer;

#[derive(Debug)]
pub(crate) struct BaseObjectManager<BaseObjectStruct> {
    objects: HashMap<NonNull<BaseObjectStruct>, BaseObjectContainer<BaseObjectStruct>>,
}

impl<BaseObjectStruct> BaseObjectManager<BaseObjectStruct> {
    pub fn add(
        &mut self,
        ptr: NonNull<BaseObjectStruct>,
        base_object: BaseObjectContainer<BaseObjectStruct>,
    ) {
        self.objects.insert(ptr, base_object);
    }

    pub fn remove(&mut self, ptr: NonNull<BaseObjectStruct>) -> VoidResult {
        logger::debug!("remove ptr: {ptr:?}");
        if self.objects.remove(&ptr).is_some() {
            Ok(())
        } else {
            anyhow::bail!("unknown base object")
        }
    }

    pub fn remove_externally(&mut self, ptr: NonNull<BaseObjectStruct>) -> VoidResult {
        if let Some(obj) = self.objects.remove(&ptr) {
            obj.try_borrow_mut()?.clear_pointers();
            Ok(())
        } else {
            anyhow::bail!("unknown base object")
        }
    }

    pub fn get_by_ptr(
        &self,
        ptr: NonNull<BaseObjectStruct>,
    ) -> Option<BaseObjectContainer<BaseObjectStruct>> {
        self.objects.get(&ptr).cloned()
    }

    pub fn has(&self, ptr: NonNull<BaseObjectStruct>) -> bool {
        self.objects.contains_key(&ptr)
    }
}

impl<BaseObjectStruct> Default for BaseObjectManager<BaseObjectStruct> {
    fn default() -> Self {
        Self {
            objects: Default::default(),
        }
    }
}
