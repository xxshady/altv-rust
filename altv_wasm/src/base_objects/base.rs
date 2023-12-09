use crate::{__imports, state::State};

/// private module for protecting ptr inside BaseObject instance
pub mod private {
    use std::marker::PhantomData;
    use altv_wasm_shared::BaseObjectPtr;

    use crate::{
        state::State,
        base_objects::objects::{BaseObjectManager, AllBaseObjects},
    };

    #[derive(Debug)]
    pub struct BaseObject<T> {
        /// must be accessed directly only in `ptr` method
        ptr: BaseObjectPtr,
        __type: PhantomData<T>,

        pub(crate) owned: bool,
    }

    impl<T> BaseObject<T> {
        /// For objects created by alt:V (or by serverside), for example player object
        pub(crate) fn internal_new_borrowed(ptr: BaseObjectPtr) -> Self {
            Self {
                ptr,
                owned: false,
                __type: PhantomData,
            }
        }

        /// For objects created by this resource, for example webview object
        pub(crate) fn internal_new_owned(ptr: BaseObjectPtr, all: &mut AllBaseObjects) -> Self {
            BaseObjectManager::add_ref(all, ptr);

            Self {
                ptr,
                owned: true,
                __type: PhantomData,
            }
        }
    }

    pub trait Ptr {
        fn ptr(&self) -> BaseObjectPtr;
    }

    impl<T> Ptr for BaseObject<T> {
        fn ptr(&self) -> BaseObjectPtr {
            let valid = State::with_base_objects_ref(|base_objects, _| {
                base_objects.all.contains_key(&self.ptr)
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

impl<T> Drop for private::BaseObject<T> {
    fn drop(&mut self) {
        if !self.owned {
            return;
        }

        let ptr = self.ptr();
        State::with_base_objects_mut(|mut objects, _| {
            objects.remove_ref(ptr);
        });
    }
}
