use dlopen::wrapper::{Container, WrapperApi};
use dlopen_derive::WrapperApi;
use std::{str::FromStr, time::Duration};

#[derive(WrapperApi)]
struct ModuleDll {
    // core, full_main_path parameters are injected using alt::res_main macro
    altMain: unsafe extern "C" fn(),
    resource_on_tick: extern "C" fn(),
    resource_start: extern "C" fn(resource_path: &str, resource_main: &str),
}

fn main() {
    println!("current dir: {:?}", std::env::current_dir().unwrap());

    let dll_path = std::path::PathBuf::from_str("./altv_module.dll").unwrap();

    let container: Container<ModuleDll> = unsafe {
        Container::load(&dll_path).unwrap_or_else(|e| {
            panic!(
                "Failed to open dll at: {}, err: {:?}",
                dll_path.display(),
                e
            )
        })
    };

    println!("calling altMain");
    unsafe { container.altMain() };

    container.resource_start(
        "C:/dev/rust/altv-xrust/target/debug",
        "example_resource.dll",
    );

    println!("starting loop");
    loop {
        std::thread::sleep(Duration::from_millis(500));
        container.resource_on_tick();
    }
}
