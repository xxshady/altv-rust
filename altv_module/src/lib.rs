mod resource_manager;

use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use once_cell::sync::OnceCell;
use resource_api::{timers, Managers, ResourceApi};
use std::{
    cell::RefCell,
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(WrapperApi)]
struct ResourceDll {
    // core, full_main_path parameters are injected using alt::res_main macro
    main: unsafe extern "C" fn(
        full_main_path: PathBuf,
        resource_api: Arc<Mutex<ResourceApi>>,
    ) -> alt::specs::CoreApplication,
}

// static mut RESOURCE_MANAGER: Mutex<ResourceManager> =
//     Mutex::new(ResourceManager { resources: vec![] });

// static mut RESOURCE_TEST: Option<Rc<alt::MainResource>> = None;

thread_local! {
    static RESOURCES: RefCell<HashMap<PathBuf, alt::specs::CoreApplication>> = RefCell::new(HashMap::new());
}

static MANAGERS_INSTANCE: OnceCell<Arc<Mutex<Managers>>> = OnceCell::new();
static RESOURCE_API: OnceCell<Arc<Mutex<ResourceApi>>> = OnceCell::new();

#[no_mangle]
#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(resource_path: &str, resource_main: &str) {
    let full_main_path = std::path::Path::new(resource_path).join(resource_main);
    dbg!(&full_main_path);

    let container: Container<ResourceDll> = unsafe {
        Container::load(&full_main_path).unwrap_or_else(|_| {
            panic!(
                "Failed to open rust resource dll at: {}",
                full_main_path.display()
            )
        })
    };

    dbg!("");

    let core_app = unsafe {
        container.main(
            full_main_path.clone(),
            Arc::clone(RESOURCE_API.get().expect("RESOURCE_API.get() failed")),
        )
    };

    dbg!("");

    // let resource_rc = Rc::new(resource);

    // unsafe {
    //     RESOURCE_MANAGER
    //         .try_lock()
    //         .unwrap()
    //         .add(full_main_path, resource_rc.clone());
    // }

    // unsafe {
    //     RESOURCE_TEST = Some(resource_rc);
    // }

    RESOURCES.with(|resources| {
        let mut resources = resources.borrow_mut();
        resources.insert(full_main_path, core_app);
    });
}

#[no_mangle]
extern "C" fn resource_on_tick() {
    println!("resource_on_tick");

    RESOURCES.with(|apps| {
        let mut apps = apps.borrow_mut();
        for app in apps.values_mut() {
            app.tick();
        }
    });

    // MANAGERS_INSTANCE
    //     .get()
    //     .unwrap()
    //     .try_lock()
    //     .unwrap()
    //     .timer_manager
    //     .try_lock()
    //     .unwrap()
    //     .process_timers();
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn altMain() -> bool {
    std::env::set_var("RUST_BACKTRACE", "1");

    println!("altMain called");

    MANAGERS_INSTANCE.set(Arc::new(Mutex::new(Managers {
        timer_manager: timers::TimerManager::instance(),
    })));

    RESOURCE_API.set(Arc::new(Mutex::new(ResourceApi {
        managers: Arc::clone(MANAGERS_INSTANCE.get().unwrap()),
    })));

    true
}
