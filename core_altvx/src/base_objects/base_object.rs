use std::{cell::RefCell, fmt::Debug, ptr::NonNull, rc::Rc};

use crate::{resource::Resource, sdk, SomeResult, VoidResult};

pub struct BaseObject<T> {
    ptr: Option<NonNull<T>>,
    base_ptr: Option<altv_sdk::BaseObjectMutPtr>,
}

impl<T> BaseObject<T> {
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
    }
}

impl<T> Debug for BaseObject<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BaseObject<T>")
    }
}

pub trait BasePtr {
    fn base_ptr(&self) -> SomeResult<altv_sdk::BaseObjectMutPtr>;
    fn raw_base_ptr(&self) -> SomeResult<altv_sdk::BaseObjectRawMutPtr>;
}

impl<T> BasePtr for BaseObject<T> {
    fn base_ptr(&self) -> SomeResult<altv_sdk::BaseObjectMutPtr> {
        if let Some(base_ptr) = self.base_ptr {
            Ok(base_ptr)
        } else {
            anyhow::bail!("base object base_ptr is none")
        }
    }

    fn raw_base_ptr(&self) -> SomeResult<altv_sdk::BaseObjectRawMutPtr> {
        Ok(self.base_ptr()?.as_ptr())
    }
}

pub trait ValidBaseObject: BasePtr {
    fn valid(&self) -> bool {
        self.base_ptr().is_ok()
    }
}

impl<T> ValidBaseObject for BaseObject<T> {}

pub(crate) type BaseObjectContainer<T> = Rc<BaseObjectWrapper<T>>;

pub struct BaseObjectWrapper<T> {
    pub(crate) value: RefCell<BaseObject<T>>,
}

impl<T> BaseObjectWrapper<T> {
    pub(crate) fn _new(
        ptr: NonNull<T>,
        base_ptr: altv_sdk::BaseObjectMutPtr,
    ) -> BaseObjectContainer<T> {
        Rc::new(Self {
            value: RefCell::new(BaseObject {
                ptr: Some(ptr),
                base_ptr: Some(base_ptr),
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

impl<T> BasePtr for BaseObjectWrapper<T> {
    fn base_ptr(&self) -> SomeResult<altv_sdk::BaseObjectMutPtr> {
        self.value.try_borrow()?.base_ptr()
    }

    fn raw_base_ptr(&self) -> SomeResult<altv_sdk::BaseObjectRawMutPtr> {
        self.value.try_borrow()?.raw_base_ptr()
    }
}
impl<T> ValidBaseObject for BaseObjectWrapper<T> {}
