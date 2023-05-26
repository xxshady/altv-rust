use std::{ops::Deref, rc::Rc};

use super::wrapper::BaseObjectWrapper;

pub struct BaseObjectContainer<T, InheritPtrs: Clone = ()>(
    pub(in super::super) Rc<BaseObjectWrapper<T, InheritPtrs>>,
);

impl<T, InheritPtrs: Clone> Deref for BaseObjectContainer<T, InheritPtrs> {
    type Target = Rc<BaseObjectWrapper<T, InheritPtrs>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, InheritPtrs: Clone> Clone for BaseObjectContainer<T, InheritPtrs> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<T, InheritPtrs: Clone> PartialEq for BaseObjectContainer<T, InheritPtrs> {
    fn eq(&self, other: &Self) -> bool {
        let Ok(borrow) = self.0.value.try_borrow() else {
            return false;
        };
        let Ok(other_borrow) = other.0.value.try_borrow() else {
            return false;
        };

        *borrow == *other_borrow
    }
}
