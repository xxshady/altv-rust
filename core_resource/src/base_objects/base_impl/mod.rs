use std::rc::Rc;

pub(crate) mod base_object;
pub(crate) mod base_ptr;
pub(crate) mod inherit_ptrs;
mod mvalue;
pub(crate) mod valid;
pub(crate) mod wrapper;

pub use self::mvalue::BaseObjectRc;

pub(crate) type BaseObjectContainer<T, InheritPtrs = ()> =
    Rc<wrapper::BaseObjectWrapper<T, InheritPtrs>>;
