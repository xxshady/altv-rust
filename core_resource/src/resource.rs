use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use core_shared::{ModuleHandlers, ResourceName};

use crate::{alt_resource, base_objects, events, script_events, timers};

thread_local! {
    pub static RESOURCE: Rc<RefCell<Option<Resource>>> =
        Rc::new(RefCell::new(None));
}

#[derive(Debug, Default)]
pub struct Resource {
    pub name: ResourceName,
    pub module_handlers: ModuleHandlers,

    pub timers: RefCell<timers::TimerManager>,
    pub timer_schedule: RefCell<timers::ScheduleState>,
    pub events: RefCell<events::EventManager>,
    pub local_script_events: RefCell<script_events::LocalEventManager>,
    pub client_script_events: RefCell<script_events::ClientEventManager>,
    pub base_objects: RefCell<base_objects::Store>,
    pub pending_base_object_destroy_or_creation: RefCell<base_objects::PendingDestroyOrCreation>,
    pub alt_resources: RefCell<alt_resource::AltResourceManager>,
}

macro_rules! with_resource {
    ($func:expr, $property_name:ident, $borrow_func:ident) => {
        paste::paste! {
            RESOURCE.with(|resource| {
                let resource = resource.borrow();
                let resource = resource.as_ref().unwrap();
                let manager = resource.[<$property_name>].[<$borrow_func>]().unwrap_or_else(|_| {
                    panic!("Failed to {} `{}`", stringify!($borrow_func), stringify!($property_name));
                });
                $func(manager, resource)
            })
        }
    };
}

macro_rules! impl_borrow_fn {
    ($property_name:ident, $full_path:path) => {
        paste::paste! {
            pub fn [<with_  $property_name _ref>]<F, R>(f: F) -> R
            where
                F: FnOnce(Ref<$full_path>, &Resource) -> R,
            {
                with_resource!(f, $property_name, try_borrow)
            }
        }
    };
}

macro_rules! impl_borrow_mut_fn {
    ($property_name:ident, $full_path:path) => {
        paste::paste! {
            pub fn [<with_  $property_name _mut>]<F, R>(f: F) -> R
            where
                F: FnOnce(RefMut<$full_path>, &Resource) -> R,
            {
                with_resource!(f, $property_name, try_borrow_mut)
            }
        }
    };
}

impl Resource {
    pub fn init(resource_name: ResourceName, module_handlers: ModuleHandlers) {
        RESOURCE.with(|container| {
            let resource = Resource {
                name: resource_name,
                module_handlers,
                ..Default::default()
            };

            resource.alt_resources.borrow_mut().init(&resource.name);

            container.replace(Some(resource));
        });
    }

    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&Resource) -> R,
    {
        RESOURCE.with(|v| f(v.borrow().as_ref().unwrap()))
    }

    fn pending_base_object_destroy_or_creation(&self) -> bool {
        self.pending_base_object_destroy_or_creation
            .try_borrow_mut()
            .is_err()
    }

    #[allow(clippy::not_unsafe_ptr_arg_deref)]
    pub fn on_base_object_create(
        &self,
        ptr: altv_sdk::BaseObjectMutPtr,
        base_object_type: altv_sdk::BaseObjectType,
    ) {
        logger::debug!("on_base_object_create {ptr:?} {base_object_type:?}");

        if self.pending_base_object_destroy_or_creation() {
            logger::debug!("pending_base_object_destroy_or_creation -> skip");
            return;
        }

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

        if self.pending_base_object_destroy_or_creation() {
            logger::debug!("pending_base_object_destroy_or_creation -> skip");
            return;
        }

        self.base_objects
            .borrow_mut()
            .on_remove(ptr, base_object_type);
    }

    impl_borrow_mut_fn!(timers, timers::TimerManager);
    impl_borrow_mut_fn!(timer_schedule, timers::ScheduleState);
    impl_borrow_mut_fn!(events, events::EventManager);
    impl_borrow_mut_fn!(local_script_events, script_events::LocalEventManager);
    impl_borrow_mut_fn!(client_script_events, script_events::ClientEventManager);
    impl_borrow_mut_fn!(base_objects, base_objects::Store);
    impl_borrow_fn!(base_objects, base_objects::Store);
    impl_borrow_mut_fn!(
        pending_base_object_destroy_or_creation,
        base_objects::PendingDestroyOrCreation
    );
    impl_borrow_fn!(alt_resources, alt_resource::AltResourceManager);
    impl_borrow_mut_fn!(alt_resources, alt_resource::AltResourceManager);
}
