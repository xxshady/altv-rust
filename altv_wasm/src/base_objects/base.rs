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

        pub(crate) fn ptr(&self) -> BaseObjectPtr {
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

impl<T> private::BaseObject<T> {
    pub fn id(&self) -> u32 {
        __imports::base_object_get_id(self.ptr())
    }
}
