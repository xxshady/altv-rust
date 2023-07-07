use std::{
    fs,
    path::Path,
    thread,
    time::{Duration, Instant},
};

use duct::cmd;

struct Files {
    altv_server: (String, String),
    crash_handler: (String, String),
    vehmodels: (String, String),
    vehmods: (String, String),
    clothes: (String, String),
    pedmodels: (String, String),
    rpfdata: (String, String),
    weaponmodels: (String, String),
}

#[tokio::main]
async fn main() {
    println!("building altv_module");
    cmd!("cargo", "build", "-p", "altv_module").run().unwrap();

    println!("building rust_resource");
    cmd!("cargo", "build", "-p", "rust_resource").run().unwrap();

    let start = if cfg!(windows) { "" } else { "lib" };
    let ext = if cfg!(windows) { ".dll" } else { ".so" };

    fs::create_dir_all("test/altv_server/modules").unwrap();
    fs::copy(
        format!("target/debug/{start}altv_module{ext}"),
        format!("test/altv_server/modules/rust-module{ext}"),
    )
    .unwrap();
    fs::create_dir_all("test/altv_server/resources/rust").unwrap();
    fs::copy(
        format!("target/debug/{start}rust_resource{ext}"),
        format!("test/altv_server/resources/rust/main{ext}"),
    )
    .unwrap();

    fs::create_dir_all("test/altv_server/resources/rust").unwrap();
    fs::write(
        "test/altv_server/resources/rust/resource.toml",
        format!(
            "\
        type = \"rs\"\n\
        main = \"main{ext}\""
        ),
    )
    .unwrap();

    let altv_server_ext = if cfg!(windows) { ".exe" } else { "" };

    let platform = if cfg!(windows) {
        "x64_win32"
    } else {
        "x64_linux"
    };

    let cdn = "https://cdn.alt-mp.com";
    let cdn_data = format!("{cdn}/data/dev/data");

    let files = Files {
        altv_server: (
            format!("{cdn}/server/dev/{platform}/altv-server{altv_server_ext}"),
            format!("test/altv_server/altv-server{altv_server_ext}"),
        ),
        crash_handler: (
            format!("{cdn}/server/dev/{platform}/altv-crash-handler{altv_server_ext}"),
            format!("test/altv_server/altv-crash-handler{altv_server_ext}"),
        ),
        vehmodels: (
            format!("{cdn_data}/vehmodels.bin"),
            "test/altv_server/data/vehmodels.bin".to_string(),
        ),
        vehmods: (
            format!("{cdn_data}/vehmods.bin"),
            "test/altv_server/data/vehmods.bin".to_string(),
        ),
        clothes: (
            format!("{cdn_data}/clothes.bin"),
            "test/altv_server/data/clothes.bin".to_string(),
        ),
        pedmodels: (
            format!("{cdn_data}/pedmodels.bin"),
            "test/altv_server/data/pedmodels.bin".to_string(),
        ),
        rpfdata: (
            format!("{cdn_data}/rpfdata.bin"),
            "test/altv_server/data/rpfdata.bin".to_string(),
        ),
        weaponmodels: (
            format!("{cdn_data}/weaponmodels.bin"),
            "test/altv_server/data/weaponmodels.bin".to_string(),
        ),
    };

    println!("downloading server files...");
    download_server_files(&files).await;

    println!("running altv server");
    let (_, server_bin) = files.altv_server;

    if cfg!(unix) {
        cmd!("chmod", "+x", &server_bin).run().unwrap();
    }

    let server_dir = Path::new(&server_bin)
        .parent()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let altv_server = cmd!(&server_bin)
        .dir(&server_dir)
        .start()
        .unwrap_or_else(|e| panic!("failed to start altv server at: {server_bin:?}, error: {e}"));

    let start = Instant::now();
    loop {
        thread::sleep(Duration::from_millis(200));

        let result = altv_server.try_wait();
        match result {
            Ok(None) => {
                if start.elapsed() >= Duration::from_secs(30) {
                    panic!("altv server process did not exit in 30 seconds");
                }

                // https://youtu.be/pLJTfLumkGw
                let log = fs::read(format!("{server_dir}/server.log")).unwrap();
                if String::from_utf8_lossy(&log).contains("Stopped resource rust") {
                    println!("rust resource stopped, killing altv server");
                    altv_server.kill().unwrap();
                    break;
                }
            }
            Ok(Some(_)) => {
                println!("altv server process stopped successfully");
                std::process::exit(0);
            }
            Err(e) => {
                panic!("altv server process CRASHED, error: {e:?}");
            }
        }
    }
}

async fn download_server_files(files: &Files) {
    let (from, to) = &files.altv_server;
    let altv_server = download_file(from, to);
    let (from, to) = &files.crash_handler;
    let crash_handler = download_file(from, to);
    let (from, to) = &files.vehmodels;
    let veh_models = download_file(from, to);
    let (from, to) = &files.vehmods;
    let veh_mods = download_file(from, to);
    let (from, to) = &files.clothes;
    let clothes = download_file(from, to);
    let (from, to) = &files.pedmodels;
    let ped_models = download_file(from, to);
    let (from, to) = &files.rpfdata;
    let rpf_data = download_file(from, to);
    let (from, to) = &files.weaponmodels;
    let weapon_models = download_file(from, to);

    tokio::join!(
        altv_server,
        crash_handler,
        veh_models,
        veh_mods,
        clothes,
        ped_models,
        rpf_data,
        weapon_models,
    );

    async fn download_file(from: &str, to: &str) {
        println!("starting: {from}");

        let res = reqwest::get(from).await.unwrap();
        let bytes = res.bytes().await.unwrap();

        let parent = Path::new(to).parent().unwrap();
        fs::create_dir_all(parent).unwrap();
        fs::write(to, bytes).unwrap();

        println!("downloaded: {to}");
    }
}
