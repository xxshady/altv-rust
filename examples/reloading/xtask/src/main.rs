use std::{
  env::{
    self,
    consts::{DLL_PREFIX, DLL_SUFFIX},
  },
  fs,
  process::Command,
};

fn main() {
  let script = env::args()
    .nth(1)
    .unwrap_or_else(|| panic!("expected `cargo task <script>`"));
  match script.as_str() {
    "build" => build(),
    "rebuild" => rebuild(),
    script => {
      panic!("unknown script: {script}");
    }
  }
}

fn build() {
  fs::create_dir_all("altv_server/resources/main").unwrap();
  fs::copy("resource.toml", "altv_server/resources/main/resource.toml").unwrap();
  fs::copy("server.toml", "altv_server/server.toml").unwrap();

  build_resource();
  cmd!("cargo", "altvup", "release", "--reloading"; current_dir: "altv_server");
}

fn rebuild() {
  build_resource();
}

fn build_resource() {
  cmd!(
    "cargo",
    "build",
    "--package",
    "resource",
    "--features",
    "reloading"
  );
  let resource_file_name = format!("{DLL_PREFIX}resource{DLL_SUFFIX}");
  fs::copy(
    format!("target/debug/{resource_file_name}"),
    "altv_server/resources/main/main.module",
  )
  .unwrap();

  if cfg!(target_os = "windows") {
    fs::copy(
      format!("target/debug/resource.pdb"),
      "altv_server/resources/main/resource.pdb",
    )
    .unwrap();
  }
}

macro_rules! cmd_impl {
  ( $program:expr $(, $arg:expr )* $(; current_dir: $current_dir:expr )? ) => ({
    let args: &[String] = &[ $( $arg.clone().into(), )* ];

    let full_command = {
      let args = args.join(" ");
      let args = if args.is_empty() {
        "".to_owned()
      } else {
        format!(" {args}")
      };

      format!("`{}{args}`", $program)
    };
    println!("running {full_command}");

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
