use libloading::Library;
use std::{str::FromStr, time::Duration};

fn main() {
    unsafe {
        println!("current dir: {:?}", std::env::current_dir().unwrap());

        let dll_path = std::path::PathBuf::from_str("./altv_module.dll").unwrap();

        let lib = Library::new(dll_path.clone()).unwrap();
        let altMain: unsafe extern "C" fn() = *lib.get(b"altMain\0").unwrap();

        println!("calling altMain");
        altMain();
        println!("called altMain");

        let resource_start: extern "C" fn(resource_path: &str, resource_main: &str) =
            *lib.get(b"resource_start\0").unwrap();

        println!("calling resource_start");

        resource_start(
            "C:/dev/rust/altv-xrust/target/debug",
            "example_resource.dll",
        );

        let resource_on_tick: extern "C" fn() = *lib.get(b"resource_on_tick\0").unwrap();

        println!("starting loop");
        loop {
            std::thread::sleep(Duration::from_millis(500));
            resource_on_tick();
        }
    }
}
