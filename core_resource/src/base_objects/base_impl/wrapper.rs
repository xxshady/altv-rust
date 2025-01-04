use std::{
  cell::RefCell,
  ptr::NonNull,
  rc::Rc,
  sync::{Arc, RwLock},
};

use super::{base_object::BaseObject, BaseObjectContainer};
use crate::{
  meta::base_object::{normal_meta::NormalBaseObjectMeta, synced_meta::SyncedBaseObjectMeta},
  resource::Resource,
  SomeResult, VoidResult,
};

pub(crate) type BaseObjectWrapperRc<T, InheritPtrs = ()> = Arc<BaseObjectWrapper<T, InheritPtrs>>;

pub struct BaseObjectWrapper<T, InheritPtrs: Clone = ()> {
  pub(crate) value: RwLock<BaseObject<T, InheritPtrs>>,
}

impl<T, InheritPtrs: Clone> BaseObjectWrapper<T, InheritPtrs> {
  pub(crate) fn _new(
    ptr: NonNull<T>,
    base_ptr: altv_sdk::BaseObjectMutPtr,
    inherit_ptrs: InheritPtrs,
  ) -> BaseObjectContainer<T, InheritPtrs> {
    BaseObjectContainer(Arc::new(Self {
      value: RwLock::new(BaseObject {
        ptr: Some(ptr),
        base_ptr: Some(base_ptr),
        inherit_ptrs: Some(inherit_ptrs),
      }),
    }))
  }

  pub(crate) fn ptr(&self) -> SomeResult<NonNull<T>> {
    Ok(self.value.read().unwrap().ptr()?)
  }

  pub(crate) fn raw_ptr(&self) -> SomeResult<*mut T> {
    self.value.read().unwrap().raw_ptr()
  }

  pub(crate) fn internal_destroy(&self) -> VoidResult {
    Resource::with_pending_base_object_destroy_or_creation_mut(|_, _| {
      self.value.write().unwrap().internal_destroy()
    })?;
    Ok(())
  }

  pub fn id(&self) -> SomeResult<u32> {
    self.value.read().unwrap().id()
  }
}

impl<T, InheritPtrs: Clone> NormalBaseObjectMeta<T, InheritPtrs>
  for BaseObjectWrapper<T, InheritPtrs>
{
}

impl<T, InheritPtrs: Clone> SyncedBaseObjectMeta<T, InheritPtrs>
  for BaseObjectWrapper<T, InheritPtrs>
{
}
