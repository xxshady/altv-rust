use std::{ops::Deref, rc::Rc};

use super::wrapper::BaseObjectWrapper;

pub struct BaseObjectContainer<T, InheritPtrs: Clone = ()>(
    pub(super) Rc<BaseObjectWrapper<T, InheritPtrs>>,
);

impl<T, InheritPtrs: Clone> Deref for BaseObjectContainer<T, InheritPtrs> {
    type Target = BaseObjectWrapper<T, InheritPtrs>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, InheritPtrs: Clone> Clone for BaseObjectContainer<T, InheritPtrs> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}
