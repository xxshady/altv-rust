use crate::__imports;

/// private module for protecting ptr inside BaseObject instance
pub mod private {
    use std::marker::PhantomData;
    use altv_wasm_shared::BaseObjectPtr;

    use crate::state::State;

    #[derive(Debug)]
    pub struct BaseObject<T> {
        /// must be accessed directly only in `ptr` method
        ptr: BaseObjectPtr,
        __type: PhantomData<T>,
    }

    impl<T> BaseObject<T> {
        pub(crate) fn new(ptr: BaseObjectPtr) -> Self {
            Self {
                ptr,
                __type: PhantomData {},
            }
        }
    }

    pub trait Ptr {
        fn ptr(&self) -> BaseObjectPtr;
    }

    impl<T> Ptr for BaseObject<T> {
        fn ptr(&self) -> BaseObjectPtr {
            let valid = State::with_base_objects_ref(|base_objects, _| {
                base_objects.all.contains(&self.ptr)
            });

            assert!(
                valid,
                "{:?} instance is invalid (perhaps it was destroyed in another script resource?)",
                self.__type
            );

            self.ptr
        }
    }

    #[derive(Debug)]
    pub struct BaseObjectLocked<'m, T, M> {
        /// must be accessed directly only in `ptr` method
        ptr: BaseObjectPtr,
        /// borrows manager to lock it
        _manager: &'m M,
        __type: PhantomData<T>,
    }

    impl<'m, T, M> BaseObjectLocked<'m, T, M> {
        pub(crate) fn new(ptr: BaseObjectPtr, manager: &'m M) -> Self {
            Self {
                ptr,
                _manager: manager,
                __type: PhantomData {},
            }
        }
    }

    impl<'m, T, M> Ptr for BaseObjectLocked<'m, T, M> {
        fn ptr(&self) -> BaseObjectPtr {
            let valid = State::with_base_objects_ref(|base_objects, _| {
                base_objects.all.contains(&self.ptr)
            });

            assert!(
                valid,
                "{:?} instance is invalid (perhaps it was destroyed in another script resource?)",
                self.__type
            );

            self.ptr
        }
    }
}

use private::Ptr;

impl<T> private::BaseObject<T> {
    pub fn id(&self) -> u32 {
        __imports::base_object_get_id(self.ptr())
    }
}
