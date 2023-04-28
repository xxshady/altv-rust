use std::{cell::RefCell, fmt::Debug, ptr::NonNull, rc::Rc};

use super::meta::impl_meta_type_for;
use crate::{resource::Resource, sdk, SomeResult, VoidResult};

pub struct BaseObject<T, InheritPtrs: Clone> {
    ptr: Option<NonNull<T>>,
    base_ptr: Option<altv_sdk::BaseObjectMutPtr>,

    /// pointers of inherited classes between `ptr` and `base_ptr` <br>
    /// for example inherited classes of Vehicle would be: WorldObject, Entity
    inherit_ptrs: Option<InheritPtrs>,
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

pub trait BaseObjectInheritPtrs<InheritPtrs> {
    fn inherit_ptrs(&self) -> SomeResult<InheritPtrs>;
}

impl<T, InheritPtrs: Clone> BaseObjectInheritPtrs<InheritPtrs> for BaseObject<T, InheritPtrs> {
    fn inherit_ptrs(&self) -> SomeResult<InheritPtrs> {
        self.inherit_ptrs
            .clone()
            .ok_or(anyhow::anyhow!("base object inherit_ptrs is none"))
    }
}

pub trait ValidBaseObject: BasePtr {
    fn valid(&self) -> bool {
        self.base_ptr().is_ok()
    }
}

impl<T, InheritPtrs: Clone> ValidBaseObject for BaseObject<T, InheritPtrs> {}

pub(crate) type BaseObjectContainer<T, InheritPtrs = ()> = Rc<BaseObjectWrapper<T, InheritPtrs>>;

pub struct BaseObjectWrapper<T, InheritPtrs: Clone = ()> {
    pub(crate) value: RefCell<BaseObject<T, InheritPtrs>>,
}

impl<T, InheritPtrs: Clone> BaseObjectWrapper<T, InheritPtrs> {
    pub(crate) fn _new(
        ptr: NonNull<T>,
        base_ptr: altv_sdk::BaseObjectMutPtr,
        inherit_ptrs: InheritPtrs,
    ) -> BaseObjectContainer<T, InheritPtrs> {
        Rc::new(Self {
            value: RefCell::new(BaseObject {
                ptr: Some(ptr),
                base_ptr: Some(base_ptr),
                inherit_ptrs: Some(inherit_ptrs),
            }),
        })
    }

    pub(crate) fn ptr(&self) -> SomeResult<NonNull<T>> {
        self.value.try_borrow()?.ptr()
    }

    pub(crate) fn raw_ptr(&self) -> SomeResult<*mut T> {
        self.value.try_borrow()?.raw_ptr()
    }

    pub(crate) fn internal_destroy(&self) -> VoidResult {
        self.value.try_borrow_mut()?.internal_destroy()
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

impl<T, InheritPtrs: Clone> BaseObjectInheritPtrs<InheritPtrs>
    for BaseObjectWrapper<T, InheritPtrs>
{
    fn inherit_ptrs(&self) -> SomeResult<InheritPtrs> {
        self.value.try_borrow()?.inherit_ptrs()
    }
}

impl<T, InheritPtrs: Clone> ValidBaseObject for BaseObjectWrapper<T, InheritPtrs> {}

impl_meta_type_for!(
    Meta,
    BaseObjectWrapper<T, InheritPtrs>,
    sdk::IBaseObject,
    BaseObjectWrapper::raw_base_ptr,
    @generics: [T, InheritPtrs: Clone,]
);

impl_meta_type_for!(
    SyncedMeta,
    BaseObjectWrapper<T, InheritPtrs>,
    sdk::IBaseObject,
    BaseObjectWrapper::raw_base_ptr,
    @generics: [T, InheritPtrs: Clone,]
);
