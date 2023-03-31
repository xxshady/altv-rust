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
    pub base_objects: RefCell<base_object::BaseObjectManager>,
    pub base_object_deletion: RefCell<base_object::PendingBaseObjectDeletion>,
    pub base_object_creation: RefCell<base_object::PendingBaseObjectCreation>,
    pub events: RefCell<events::EventManager>,
    pub local_script_events: RefCell<script_events::LocalEventManager>,
    pub client_script_events: RefCell<script_events::ClientEventManager>,
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
        raw_ptr: altv_sdk::IBaseObjectMutPtr,
        base_object_type: altv_sdk::BaseObjectType,
    ) {
        if self.base_object_creation.try_borrow_mut().is_err() {
            logger::debug!("on_base_object_create pending creation, skip");
            return;
        }

        use altv_sdk::BaseObjectType::*;
        let base_object: base_object::BaseObjectContainer = match base_object_type {
            Colshape => {
                let col_shape = col_shape::create_col_shape_container(raw_ptr);
                self.col_shape_base_object_map
                    .borrow_mut()
                    .add_base_object(Rc::clone(&col_shape));

                col_shape
            }
            _ => todo!(),
        };

        self.base_objects
            .borrow_mut()
            .on_create(raw_ptr, base_object);
    }

    pub fn on_base_object_destroy(&self, raw_ptr: altv_sdk::IBaseObjectMutPtr) {
        if self.base_object_deletion.try_borrow_mut().is_err() {
            logger::debug!("on_base_object_destroy pending deletion, skip");
            return;
        }

        let remove_entity_from_pool = |base_object_borrow: &Ref<dyn base_object::BaseObject>| {
            self.entities
                .borrow_mut()
                .on_destroy(base_object_borrow.ptr().to_entity().unwrap());
        };

        let mut base_objects = self.base_objects.borrow_mut();
        let base_object = base_objects.get_by_raw_ptr(raw_ptr);
        if let Some(base_object) = base_object {
            let base_object_borrow = base_object.borrow();

            use altv_sdk::BaseObjectType::*;
            match base_object_borrow.base_type() {
                Colshape => {
                    self.col_shape_base_object_map
                        .borrow_mut()
                        .remove_base_object(base_object_borrow.ptr().get().unwrap());
                }
                _ => todo!(),
            };
            drop(base_object_borrow);
            base_objects.on_destroy(Rc::clone(&base_object));
        } else {
            logger::error!("on_base_object_destroy unknown base object: {raw_ptr:?}");
        }
    }

    impl_borrow_mut_fn!(timers, timers::TimerManager);
    impl_borrow_mut_fn!(timer_schedule, timers::ScheduleState);
    impl_borrow_mut_fn!(base_objects, base_object::BaseObjectManager);
    impl_borrow_mut_fn!(base_object_deletion, base_object::PendingBaseObjectDeletion);
    impl_borrow_mut_fn!(base_object_creation, base_object::PendingBaseObjectCreation);
    impl_borrow_mut_fn!(events, events::EventManager);
    impl_borrow_mut_fn!(local_script_events, script_events::LocalEventManager);
    impl_borrow_mut_fn!(client_script_events, script_events::ClientEventManager);
}
