use std::cell::{RefCell, RefMut, Ref};

use crate::{timers, base_objects};

thread_local! {
    pub(crate) static STATE: RefCell<Option<State>> =
        RefCell::new(None);
}

#[derive(Debug, Default)]
pub(crate) struct State {
    pub(crate) timers: RefCell<timers::TimerManager>,
    pub(crate) timer_schedule: RefCell<timers::ScheduleState>,
    pub(crate) base_objects: RefCell<base_objects::manager::BaseObjectManager>,
}

macro_rules! with_state {
    ($func:expr, $property_name:ident, $borrow_func:ident) => {
        paste::paste! {
            STATE.with(|state| {
                let state = state.borrow();
                let state = state.as_ref().unwrap();
                let manager = state.[<$property_name>].[<$borrow_func>]().unwrap_or_else(|_| {
                    panic!("Failed to {} `{}`", stringify!($borrow_func), stringify!($property_name));
                });
                $func(manager, state)
            })
        }
    };
}

macro_rules! impl_borrow_fn {
    ($property_name:ident, $full_path:path) => {
        paste::paste! {
            pub fn [<with_  $property_name _ref>]<F, R>(f: F) -> R
            where
                F: FnOnce(Ref<$full_path>, &State) -> R,
            {
                with_state!(f, $property_name, try_borrow)
            }
        }
    };
}

macro_rules! impl_borrow_mut_fn {
    ($property_name:ident, $full_path:path) => {
        paste::paste! {
            pub fn [<with_  $property_name _mut>]<F, R>(f: F) -> R
            where
                F: FnOnce(RefMut<$full_path>, &State) -> R,
            {
                with_state!(f, $property_name, try_borrow_mut)
            }
        }
    };
}

impl State {
    pub fn init() {
        STATE.with(|container| {
            let state = State {
                ..Default::default()
            };

            container.replace(Some(state));
        });
    }

    pub fn with<F, R>(f: F) -> R
    where
        F: FnOnce(&State) -> R,
    {
        STATE.with(|v| f(v.borrow().as_ref().unwrap()))
    }

    impl_borrow_mut_fn!(timers, timers::TimerManager);
    impl_borrow_mut_fn!(timer_schedule, timers::ScheduleState);
    impl_borrow_mut_fn!(base_objects, base_objects::manager::BaseObjectManager);
    impl_borrow_fn!(base_objects, base_objects::manager::BaseObjectManager);
}
