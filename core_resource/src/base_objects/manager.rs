use crate::VoidResult;
use std::{collections::HashMap, fmt::Debug, ptr::NonNull};

use super::base_object::BaseObjectContainer;

pub(crate) struct BaseObjectManager<T> {
    objects: HashMap<NonNull<T>, BaseObjectContainer<T>>,
}

impl<T> BaseObjectManager<T> {
    pub fn add(&mut self, ptr: NonNull<T>, base_object: BaseObjectContainer<T>) {
        self.objects.insert(ptr, base_object);
    }

    pub fn remove(&mut self, ptr: NonNull<T>) -> VoidResult {
        logger::debug!("remove ptr: {ptr:?}");
        if self.objects.remove(&ptr).is_some() {
            Ok(())
        } else {
            anyhow::bail!("unknown base object")
        }
    }

    pub fn remove_externally(&mut self, ptr: NonNull<T>) -> VoidResult {
        if let Some(obj) = self.objects.remove(&ptr) {
            obj.value.try_borrow_mut()?.clear_pointers();
            Ok(())
        } else {
            anyhow::bail!("unknown base object")
        }
    }

    pub fn get_by_ptr(&self, ptr: NonNull<T>) -> Option<BaseObjectContainer<T>> {
        self.objects.get(&ptr).cloned()
    }

    pub fn has(&self, ptr: NonNull<T>) -> bool {
        self.objects.contains_key(&ptr)
    }

    pub fn all(&self) -> Vec<BaseObjectContainer<T>> {
        // TODO: dont clone it on every access?
        self.objects.values().cloned().collect()
    }
}

impl<T> Default for BaseObjectManager<T> {
    fn default() -> Self {
        Self {
            objects: Default::default(),
        }
    }
}

impl<T> Debug for BaseObjectManager<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BaseObjectManager<T>")
    }
}
