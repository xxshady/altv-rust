use std::{
    rc::Rc,
    cell::{RefCell, RefMut},
};

use crate::timers;

thread_local! {
    pub static RESOURCE: Rc<RefCell<Option<Resource>>> =
        Rc::new(RefCell::new(None));
}

#[derive(Debug, Default)]
pub struct Resource {
    pub timers: RefCell<timers::TimerManager>,
    pub timer_schedule: RefCell<timers::ScheduleState>,
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
    pub fn init() {
        RESOURCE.with(|container| {
            let resource = Resource {
                ..Default::default()
            };

            container.replace(Some(resource));
        });
    }

    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&Resource) -> R,
    {
        RESOURCE.with(|v| f(v.borrow().as_ref().unwrap()))
    }

    impl_borrow_mut_fn!(timers, timers::TimerManager);
    impl_borrow_mut_fn!(timer_schedule, timers::ScheduleState);
}
