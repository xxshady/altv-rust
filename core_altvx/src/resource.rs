use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use core_shared::{ModuleHandlers, ResourceMainPath};

use crate::{base_object, col_shape, events, script_events, timers};

thread_local! {
    pub static RESOURCE: Rc<RefCell<Option<Resource>>> =
        Rc::new(RefCell::new(None));
}

#[derive(Debug, Default)]
pub struct Resource {
    pub full_main_path: ResourceMainPath,
    pub module_handlers: ModuleHandlers,

    pub timers: RefCell<timers::TimerManager>,
    pub timer_schedule: RefCell<timers::ScheduleState>,
    pub events: RefCell<events::EventManager>,
    pub local_script_events: RefCell<script_events::LocalEventManager>,
    pub base_objects: RefCell<base_object::Store>,
    pub pending_base_object_destroy: RefCell<base_object::PendingDestroy>,
}

macro_rules! with_resource {
    ($func: expr, $property_name: ident, $borrow_func: ident) => {
        paste::paste! {
            RESOURCE.with(|resource| {
                let resource = resource.borrow();
                let resource = resource.as_ref().unwrap();
                $func(resource.[<$property_name>].[<$borrow_func>](), resource)
            })
        }
    };
}

macro_rules! impl_borrow_fn {
    ($property_name: ident, $full_path: path) => {
        paste::paste! {
            pub fn [<with_  $property_name _ref>]<F, R>(f: F) -> R
            where
                F: FnOnce(Ref<$full_path>, &Resource) -> R,
            {
                with_resource!(f, $property_name, borrow)
            }
        }
    };
}

macro_rules! impl_borrow_mut_fn {
    ($property_name: ident, $full_path: path) => {
        paste::paste! {
            pub fn [<with_  $property_name _mut>]<F, R>(f: F) -> R
            where
                F: FnOnce(RefMut<$full_path>, &Resource) -> R,
            {
                with_resource!(f, $property_name, borrow_mut)
            }
        }
    };
}

impl Resource {
    pub fn init(full_main_path: ResourceMainPath, module_handlers: ModuleHandlers) {
        RESOURCE.with(|container| {
            container.replace(Some(Resource {
                full_main_path,
                module_handlers,
                ..Default::default()
            }));
        });
    }

    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&Resource) -> R,
    {
        RESOURCE.with(|v| f(v.borrow().as_ref().unwrap()))
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn on_base_object_create(
        &self,
        ptr: altv_sdk::BaseObjectMutPtr,
        base_object_type: altv_sdk::BaseObjectType,
    ) {
        logger::debug!("on_base_object_create {ptr:?} {base_object_type:?}");

        self.base_objects
            .borrow_mut()
            .on_create(ptr, base_object_type);
    }

    pub fn on_base_object_destroy(
        &self,
        ptr: altv_sdk::BaseObjectMutPtr,
        base_object_type: altv_sdk::BaseObjectType,
    ) {
        logger::debug!("on_base_object_destroy {ptr:?} {base_object_type:?}");

        if self.pending_base_object_destroy.try_borrow_mut().is_err() {
            logger::debug!("base object destroying -> skip");
            return;
        }

        self.base_objects
            .borrow_mut()
            .on_remove(ptr, base_object_type);
        // let remove_entity_from_pool = |base_object_borrow: &Ref<dyn base_object::BaseObject>| {
        //     self.entities
        //         .borrow_mut()
        //         .on_destroy(base_object_borrow.ptr().to_entity().unwrap());
        // };

        // let mut base_objects = self.base_objects.borrow_mut();
        // let base_object = base_objects.get_by_raw_ptr(raw_ptr);
        // if let Some(base_object) = base_object {
        //     let base_object_borrow = base_object.borrow();

        //     use altv_sdk::BaseObjectType::*;
        //     match base_object_borrow.base_type() {
        //         Colshape => {
        //             self.col_shape_base_object_map
        //                 .borrow_mut()
        //                 .remove_base_object(base_object_borrow.ptr().get().unwrap());
        //         }
        //         _ => todo!(),
        //     };
        //     drop(base_object_borrow);
        //     base_objects.on_destroy(Rc::clone(&base_object));
        // } else {
        //     logger::error!("on_base_object_destroy unknown base object: {raw_ptr:?}");
        // }
    }

    impl_borrow_mut_fn!(timers, timers::TimerManager);
    impl_borrow_mut_fn!(timer_schedule, timers::ScheduleState);
    impl_borrow_mut_fn!(events, events::EventManager);
    impl_borrow_mut_fn!(local_script_events, script_events::LocalEventManager);
    impl_borrow_mut_fn!(base_objects, base_object::Store);
    impl_borrow_mut_fn!(pending_base_object_destroy, base_object::PendingDestroy);
}
