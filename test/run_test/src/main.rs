use std::{
  env, fs,
  path::Path,
  process::Command,
  thread,
  time::{Duration, Instant},
};

fn main() {
  println!("building without features");
  cmd!("cargo", "build");

  println!("building with all features");
  cmd!("cargo", "build", "--all-features");

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
    "\
        type = \"rs\"\n\
        main = \"main\"\n",
  )
  .unwrap();

  let altv_server_ext = if cfg!(windows) { ".exe" } else { "" };

  let branch = env::var("ALTV_BRANCH").unwrap();
  // TODO: remove dev branch?
  let branch = match branch.as_str() {
    "release" | "rc" | "dev" => branch,
    _ => {
      println!("fallback to altv dev branch");
      "dev".to_string()
    }
  };
  println!("using altv branch: {branch}");

  let altv_server_bin = format!("test/altv_server/altv-server{altv_server_ext}");
  let crash_handler_bin = format!("test/altv_server/altv-crash-handler{altv_server_ext}");

  println!("installing altvup");
  cmd!("cargo", "install", "--path", "altvup");

  println!("running altvup");
  // TODO: add param for local directory (to not download source code from github)
  cmd!("cargo", "altvup", branch, "--dont-compile"; current_dir: "test/altv_server");

  println!("running altv server");
  if cfg!(unix) {
    cmd!("chmod", "+x", &altv_server_bin);
    cmd!("chmod", "+x", crash_handler_bin);
  }

  let server_dir = Path::new(&altv_server_bin)
    .parent()
    .unwrap()
    .to_string_lossy()
    .to_string();

  let mut altv_server = Command::new(&altv_server_bin).spawn().unwrap();

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
        let log = fs::read(format!("{server_dir}/server.log"));
        let Ok(log) = log else {
          println!("server.log is not available yet...");
          continue;
        };
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

macro_rules! cmd_impl {
  ( $program:expr $(, $arg:expr )* $(; current_dir: $current_dir:expr )? ) => ({
    let status = Command::new($program)
      $( .arg($arg) )*
      $( .current_dir($current_dir) )?
      .status()
      .unwrap();

    if !status.success() {
      panic!("Failed to execute {}", $program);
    }
  });
}

use cmd_impl as cmd;
