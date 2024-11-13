use std::{
  fs,
  path::{Path, PathBuf},
  process::Command,
};

use colored::*;
use flate2::read::GzDecoder;
use serde::Deserialize;
use anyhow::{bail, Context};

use crate::shared::find_cli_param;

const DEFAULT_SOURCE_DIRECTORY: &str = ".altvup-src";
const SOURCE_ASSET: &str = "source.tar.gz";
const RELEASES_URL: &str = "https://api.github.com/repos/xxshady/altv-rust/releases";
const RUST_MODULE_PREFIX: &str = if cfg!(windows) { "" } else { "lib" };
const RUST_MODULE_EXT: &str = if cfg!(windows) { ".dll" } else { ".so" };

#[derive(Debug, Deserialize)]
struct GithubAsset {
  name: String,
  browser_download_url: String,
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
  name: String,
  assets: Vec<GithubAsset>,
}

fn get_releases(agent: &ureq::Agent) -> anyhow::Result<Vec<GithubRelease>> {
  println!(
    "Loading releases data from {} repo",
    "altv-rust".bright_green()
  );

  agent
    .get(RELEASES_URL)
    .call()
    .with_context(|| format!("Failed to get github releases data from: {RELEASES_URL}"))?
    .into_json()
    .with_context(|| format!("Failed to convert github releases data to json from: {RELEASES_URL}"))
}

fn compile_module_from_release(
  agent: &ureq::Agent,
  release: GithubRelease,
  cli_args: &[String],
) -> anyhow::Result<()> {
  let source_code = download_source_code_from_release(agent, release)?;
  let decoder = GzDecoder::new(&source_code[..]);
  let mut archive = tar::Archive::new(decoder);

  let src_dir = get_src_dir_for_rust_module(cli_args)?;

  println!(
    "Unpacking altv-rust source code to directory: {}\n{}",
    src_dir.display(),
    "note: it will be deleted after rust-module compilation".bright_black(),
  );

  archive.unpack(&src_dir)?;

  let result = compile_rust_module(&src_dir);

  if let Err(e) = remove_src_dir(&src_dir) {
    println!("Error: {}", e);
  }

  result
}

fn get_src_dir_for_rust_module(cli_args: &[String]) -> anyhow::Result<PathBuf> {
  let src_dir = find_cli_param(cli_args, "src-dir");
  let src_dir: PathBuf = match src_dir {
    Some(src_dir) => {
      println!(
        "Using custom directory for altv-rust source code: {}\n{}",
        src_dir.bright_magenta(),
        "note: --src-dir does not support paths with spaces yet".bright_black()
      );
      src_dir.into()
    }
    None => DEFAULT_SOURCE_DIRECTORY.into(),
  };

  if !fs::exists(&src_dir)? {
    Ok(src_dir)
  } else {
    let notes = if src_dir.to_str() == Some(DEFAULT_SOURCE_DIRECTORY) {
      format!("{}\n{}",
          "note: use `cargo altvup <branch> --src-dir=<custom directory>` to specify custom directory for altv-rust source code".bright_black(),
          "warning: --src-dir does not support paths with spaces yet".bright_black()
        )
    } else {
      String::new()
    };

    bail!(
      "{}\n{notes}",
      format!("{} directory must not exist", src_dir.display()).bright_red(),
    );
  }
}

fn download_source_code_from_release(
  agent: &ureq::Agent,
  release: GithubRelease,
) -> anyhow::Result<Vec<u8>> {
  println!(
    "Downloading altv-rust source code from release: {}",
    release.name.bright_blue()
  );

  let asset = release
    .assets
    .iter()
    .find(|v| v.name == SOURCE_ASSET)
    .with_context(|| {
      format!(
        "Cannot find {SOURCE_ASSET} in release: {} of altv-rust repo",
        release.name
      )
    })?;

  let mut bytes = vec![];

  agent
    .get(&asset.browser_download_url)
    .call()
    .with_context(|| format!("Failed to download asset: {}", asset.browser_download_url))?
    .into_reader()
    .read_to_end(&mut bytes)
    .with_context(|| {
      format!(
        "Failed to get bytes of asset: {}",
        asset.browser_download_url
      )
    })?;

  Ok(bytes)
}

fn remove_src_dir(src_dir: &PathBuf) -> anyhow::Result<()> {
  println!(
    "{}",
    format!("Removing {} directory", src_dir.display()).bright_black()
  );
  fs::remove_dir_all(src_dir)
    .with_context(|| format!("Failed to remove {} directory", src_dir.display()))
}

fn compile_rust_module(src_dir: &Path) -> anyhow::Result<()> {
  let rust_module_lib = rust_module_lib_name();
  println!("Compiling {rust_module_lib}");

  let status = Command::new("cargo")
    .arg("build")
    .arg("--release")
    .current_dir(src_dir.join("altv_module"))
    .status()
    .context("Failed to run `cargo build --release` for rust-module")?;

  if !status.success() {
    bail!("Failed to build rust-module binary");
  }

  fs::create_dir_all("modules").context("Failed to create modules directory")?;

  let source_lib_path = src_dir.join(format!(
    "target/release/{RUST_MODULE_PREFIX}altv_module{RUST_MODULE_EXT}"
  ));
  let target_lib_path = rust_module_target_path();

  println!("Saving to: {}", target_lib_path.bright_cyan());

  fs::copy(source_lib_path, target_lib_path).context("Failed to copy rust-module binary")?;

  println!("{}", "Successfully saved".bright_green());

  Ok(())
}

fn download_and_compile_rust_module_from_github_releases(
  agent: &ureq::Agent,
  branch: &str,
  cli_args: &[String],
) -> anyhow::Result<()> {
  let releases = get_releases(agent)?;
  let release = releases
    .into_iter()
    .find(|v| v.name.starts_with(&format!("{branch}-v")))
    .with_context(|| format!("Cannot find altv-rust release of branch: {branch}"))?;

  compile_module_from_release(agent, release, cli_args)
}

pub fn compile(agent: &ureq::Agent, branch: &str, cli_args: &[String]) -> anyhow::Result<()> {
  let force_recompile = find_cli_param(cli_args, "force-recompile");
  if force_recompile.is_none() {
    let target_path = rust_module_target_path();
    let exists =
      fs::exists(&target_path).with_context(|| format!("Failed to check {target_path} file"))?;

    if exists {
      println!(
        "Rust module won't be installed, because {target_path} file already exists\n\
        note: use `cargo altvup <branch> --force-recompile` if you want to recompile it",
      );
      return Ok(());
    }
  }

  download_and_compile_rust_module_from_github_releases(agent, branch, cli_args)
}

fn rust_module_lib_name() -> String {
  format!("{RUST_MODULE_PREFIX}rust-module{RUST_MODULE_EXT}")
}

fn rust_module_target_path() -> String {
  format!("modules/{}", rust_module_lib_name())
}
