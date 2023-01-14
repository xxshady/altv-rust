use libloading::Library;
use std::{cell::RefCell, collections::HashMap, path::PathBuf};

type ResourceMainFn =
    unsafe extern "C" fn(
        full_main_path: PathBuf,
    ) -> Result<alt::specs::CoreApplication, Box<dyn std::error::Error>>;

thread_local! {
    static RESOURCES: RefCell<HashMap<PathBuf, alt::specs::CoreApplication>> = RefCell::new(HashMap::new());
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
extern "C" fn resource_start(resource_path: &str, resource_main: &str) {
    let full_main_path = std::path::Path::new(resource_path).join(resource_main);
    dbg!(&full_main_path);

    // let container: Container<ResourceDll> = unsafe {
    //     Container::load(&full_main_path).unwrap_or_else(|_| {
    //         panic!(
    //             "Failed to open rust resource dll at: {}",
    //             full_main_path.display()
    //         )
    //     })
    // };

    // let core_app = unsafe { container.main(full_main_path.clone()).unwrap() };

    dbg!("");
    let lib = Library::new(full_main_path.clone()).unwrap();
    let main_fn: ResourceMainFn = unsafe { *lib.get(b"main\0").unwrap() };
    let core_app = unsafe { main_fn(full_main_path.clone()) }.unwrap();
    dbg!("111");

    // TODO: better alternative to "leak"
    Box::leak(Box::new(lib));

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
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn altMain() -> bool {
    std::env::set_var("RUST_BACKTRACE", "1");

    println!("altMain called");

    true
}
