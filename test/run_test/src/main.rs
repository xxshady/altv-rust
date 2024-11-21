use std::{
  env, fs,
  path::{Path, PathBuf},
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

  let resource_dir = PathBuf::from("test/altv_server/resources/rust");

  fs::remove_dir_all(&resource_dir).unwrap();
  fs::create_dir_all(&resource_dir).unwrap();

  let dylib_file = "main.module";

  fs::copy(
    format!("target/debug/{start}rust_resource{ext}"),
    resource_dir.join(dylib_file),
  )
  .unwrap();

  fs::write(
    resource_dir.join("resource.toml"),
    format!(
      "\
        type = \"rs\"\n\
        main = \"{dylib_file}\"\n"
    ),
  )
  .unwrap();

  let altv_server_ext = if cfg!(windows) { ".exe" } else { "" };

  let branch = env::var("ALTV_BRANCH").unwrap_or("release".to_string());
  // TODO: remove dev branch?
  let branch = match branch.as_str() {
    branch @ ("release" | "rc" | "dev") => branch,
    _ => {
      println!("fallback to altv release branch");
      "release"
    }
  };
  println!("using altv branch: {branch}");

  let altv_server_bin = format!("altv-server{altv_server_ext}");
  let altv_server_path = format!("test/altv_server/{altv_server_bin}");
  let crash_handler_path = format!("test/altv_server/altv-crash-handler{altv_server_ext}");

  println!("installing altvup");
  cmd!("cargo", "install", "--path", "altvup", "--force");

  println!("running altvup");
  // TODO: add param for local directory (to not download source code from github)
  cmd!("cargo", "altvup", branch, "--dont-compile"; current_dir: "test/altv_server");

  if cfg!(unix) {
    cmd!("chmod", "+x", &altv_server_path);
    cmd!("chmod", "+x", crash_handler_path);
  }

  let server_dir = Path::new(&altv_server_path).parent().unwrap();
  let server_log_path = server_dir.join("server.log");

  println!("removing server.log");
  {
    let res = fs::remove_file(&server_log_path);
    println!("res: {res:?}");
  }

  let program = if cfg!(unix) {
    format!("./{altv_server_bin}")
  } else {
    altv_server_bin
  };

  println!("running altv server");

  let mut command: Command;
  let altv_server = if cfg!(unix) {
    command = Command::new(program);
    &mut command
  } else {
    command = Command::new("cmd");
    command.args(["/C", &program])
  };

  let mut altv_server = altv_server.current_dir(server_dir).spawn().unwrap();

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
        let log = fs::read(&server_log_path);
        let Ok(log) = log else {
          println!("server.log is not available yet...");
          continue;
        };
        if String::from_utf8_lossy(&log).contains("Stopped resource rust") {
          println!("rust resource stopped, killing altv server");

          if cfg!(unix) {
            altv_server.kill().unwrap();
          } else {
            cmd!("taskkill", "/F", "/IM", "altv-server.exe");
          }

          break;
        }
      }
      Ok(Some(_)) => {
        println!("altv server process stopped successfully");
        break;
      }
      Err(e) => {
        panic!("altv server process CRASHED, error: {e:?}");
      }
    }
  }

  // TODO: fix exit, terminal is messed up after exit for some reason
  println!("exit");
}

macro_rules! cmd_impl {
  ( $program:expr $(, $arg:expr )* $(; current_dir: $current_dir:expr )? ) => ({
    let args: &[String] = &[ $( $arg.clone().into(), )* ];

    let status = Command::new($program)
      .args(args)
      $( .current_dir($current_dir) )?
      .status()
      .unwrap_or_else(|e| {
        let args = args.join(" ");
        panic!("Failed to execute: `{} {}`, reason: {e:#?}", $program, args);
      });

    if !status.success() {
      panic!("Program: {} did not exit successfully", $program);
    }
  });
}

use cmd_impl as cmd;
