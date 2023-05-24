use super::{base_object::BaseObject, wrapper::BaseObjectWrapper};
use crate::SomeResult;

pub trait BaseObjectInheritPtrs<InheritPtrs> {
    fn inherit_ptrs(&self) -> SomeResult<InheritPtrs>;
}

impl<T, InheritPtrs: Clone> BaseObjectInheritPtrs<InheritPtrs> for BaseObject<T, InheritPtrs> {
    fn inherit_ptrs(&self) -> SomeResult<InheritPtrs> {
        self.inherit_ptrs
            .clone()
            .ok_or(anyhow::anyhow!("Base object is destroyed and cannot be used anymore (inherit_ptrs is none)"))
    }
}

impl<T, InheritPtrs: Clone> BaseObjectInheritPtrs<InheritPtrs>
    for BaseObjectWrapper<T, InheritPtrs>
{
    fn inherit_ptrs(&self) -> SomeResult<InheritPtrs> {
        self.value.try_borrow()?.inherit_ptrs()
    }
}
