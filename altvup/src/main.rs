#![deny(clippy::unwrap_used)]

use std::{fs, path::PathBuf, process::Command};

use anyhow::{bail, Context};
use colored::*;
use flate2::read::GzDecoder;
use futures_lite::future;
use reqwest::{Client, header::USER_AGENT};
use serde::Deserialize;

const DEFAULT_SOURCE_DIRECTORY: &str = ".altvup-src";
const SOURCE_ASSET: &str = "source.tar.gz";
const RELEASES_URL: &str = "https://api.github.com/repos/xxshady/altv-rust/releases";

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

fn main() -> anyhow::Result<()> {
  future::block_on(async_main())
}

async fn async_main() -> anyhow::Result<()> {
  #[cfg(windows)]
  control::set_virtual_terminal(true).expect("must never happen");

  print_altvup();

  // skipping "cargo altvup" part
  let args = std::env::args().skip(2);

  let args = args.collect::<Box<[String]>>();
  let args = &args[..];

  let [branch, others @ ..] = args else {
    bail!("Expected branch, example usage: cargo altvup release");
  };

  let src_dir: PathBuf = {
    const SRC_DIR_PARAM_START: &str = "--src-dir=";
    let src_dir_param = others
      .iter()
      .find(|arg| arg.starts_with(SRC_DIR_PARAM_START));

    if let Some(src_dir_param) = src_dir_param {
      let value = &src_dir_param[SRC_DIR_PARAM_START.len()..];
      value.into()
    } else {
      DEFAULT_SOURCE_DIRECTORY.into()
    }
  };

  let client = Client::new();

  println!(
    "Selected {} branch: {}",
    "alt:V".bright_green(),
    branch.bright_purple()
  );

  let releases = get_releases(&client).await?;
  let release = releases
    .into_iter()
    .find(|v| v.name.starts_with(&format!("{branch}-v")))
    .with_context(|| format!("Cannot find altv-rust release of branch: {branch}"))?;

  build_module_from_release(&client, release, src_dir).await?;

  print_altvup();

  Ok(())
}

async fn get_releases(client: &Client) -> anyhow::Result<Vec<GithubRelease>> {
  println!(
    "Loading releases data from {} repo",
    "altv-rust".bright_green()
  );

  let res = client
    .get(RELEASES_URL)
    .header(USER_AGENT, "rust-altv-updater")
    .send()
    .await
    .with_context(|| format!("Failed to get github releases data from: {RELEASES_URL}"))?;
  res
    .json()
    .await
    .with_context(|| format!("Failed to convert github releases data to json from: {RELEASES_URL}"))
}

async fn build_module_from_release(
  client: &Client,
  release: GithubRelease,
  src_dir: PathBuf,
) -> anyhow::Result<()> {
  let source_code = download_source_code_from_release(client, release).await?;
  let decoder = GzDecoder::new(&source_code[..]);
  let mut archive = tar::Archive::new(decoder);

  if fs::exists(&src_dir)? {
    bail!(
      "{}\n{}", 
      format!("{} directory must not exist", src_dir.display()).bright_red(),
      "note: use `cargo altvup <branch> --src-dir=<custom directory>` to specify custom directory for altv-rust source code"
    );
  }

  println!(
    "Unpacking altv-rust source code to directory: {} (it will be deleted after rust-module build)",
    src_dir.display()
  );

  archive.unpack(&src_dir)?;

  let prefix = if cfg!(windows) { "" } else { "lib" };
  let ext = if cfg!(windows) { ".dll" } else { ".so" };

  let rust_module_lib = format!("{prefix}rust-module{ext}");
  println!("Compiling {rust_module_lib}");

  let status = Command::new("cargo")
    .arg("build")
    .arg("--release")
    .current_dir(src_dir.join("altv_module"))
    .status()
    .with_context(|| "Failed to run `cargo build --release` for rust-module")?;

  if !status.success() {
    bail!("Failed to build rust-module binary");
  }

  fs::create_dir_all("modules").expect("Failed to create modules directory");

  let source_lib_path = src_dir.join(format!("target/release/{prefix}altv_module{ext}"));
  let target_lib_path = format!("modules/{rust_module_lib}");

  println!("Saving to: {}", target_lib_path.bright_cyan());

  fs::copy(source_lib_path, target_lib_path)
    .with_context(|| "Failed to copy rust-module binary")?;

  println!("{}", "Successfully saved".bright_green());

  println!("Removing {} directory", src_dir.display());
  fs::remove_dir_all(&src_dir)
    .with_context(|| format!("Failed to remove {} directory", src_dir.display()))?;

  Ok(())
}

async fn download_source_code_from_release(
  client: &Client,
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
      format!("Cannot find {SOURCE_ASSET} in release: {release:?} of altv-rust repo")
    })?;

  let bytes = client
    .get(&asset.browser_download_url)
    .send()
    .await
    .with_context(|| format!("Failed to download asset: {}", asset.browser_download_url))?
    .bytes()
    .await
    .with_context(|| {
      format!(
        "Failed to get bytes of asset: {}",
        asset.browser_download_url
      )
    })?;

  Ok(bytes.to_vec())
}

fn print_altvup() {
  println!(
    "====== {}{} ======",
    "altv".bright_green(),
    "up".bright_cyan()
  );
}
