use std::{
  collections::HashMap,
  env::consts::OS,
  fs,
  io::{self, Write},
  path::PathBuf,
};

use anyhow::{bail, Context};
use colored::Colorize;
use serde::Deserialize;
use sha1_smol::Sha1;

const CDN_ADDRESS: &str = "cdn.alt-mp.com";

type LocalPath = String;
type Sha1String = String;

#[derive(Debug)]
struct CdnFileMetadata {
  local_path: LocalPath,
  url: String,
  hash: String,
}

/// Content of update.json from alt:V CDN
#[derive(Deserialize)]
struct UpdateJson {
  #[serde(rename = "hashList")]
  hash_list: HashMap<LocalPath, Sha1String>,
}

pub fn download(agent: &ureq::Agent, branch: &str, load_jsv2: bool) -> anyhow::Result<()> {
  let files = fetch_downloadable_files(agent, branch, load_jsv2)?;
  let outdated_files = check_file_hashes(files)?;

  if outdated_files.is_empty() {
    println!("All the files seem to be {}", "up to date".bright_green());
    return Ok(());
  }

  download_files(agent, outdated_files)
}

fn check_file_hashes(files: Vec<CdnFileMetadata>) -> anyhow::Result<Vec<CdnFileMetadata>> {
  println!(
    "Checking file hashes {}",
    "(to download only outdated files)".bright_black()
  );

  let outdated_files = files
    .into_iter()
    .filter(|file| {
      let local_hash = hash_of_local_file(&file.local_path);
      match local_hash {
        Ok(FileHashResult::Exists(local_hash)) => local_hash != file.hash,
        Ok(FileHashResult::DoesNotExist) => true,
        Err(e) => {
          println!(
            "{}",
            format!(
              "Failed to read {}, it will be re-downloaded from alt:V CDN\n\
              cause: {e:#}",
              file.local_path
            )
            .bright_black()
          );
          true
        }
      }
    })
    .collect();
  Ok(outdated_files)
}

fn fetch_downloadable_files(
  agent: &ureq::Agent,
  branch: &str,
  load_jsv2: bool,
) -> anyhow::Result<Vec<CdnFileMetadata>> {
  println!("Requesting list of files from alt:V CDN");

  let platform = match OS {
    "windows" => "x64_win32",
    "linux" => "x64_linux",
    os => {
      bail!("Unsupported OS: {os}");
    }
  };

  let mut files = Vec::<CdnFileMetadata>::new();

  let data_files_url = format!("https://{CDN_ADDRESS}/data/{branch}");
  let server_binaries_url = format!("https://{CDN_ADDRESS}/server/{branch}/{platform}");

  fetch_file_metadata_from(&data_files_url, agent, &mut files)?;
  fetch_file_metadata_from(&server_binaries_url, agent, &mut files)?;

  if load_jsv2 {
    let js_module_v2_url = format!("https://{CDN_ADDRESS}/js-module-v2/{branch}/{platform}");
    fetch_file_metadata_from(&js_module_v2_url, agent, &mut files)?;
  }

  Ok(files)
}

fn fetch_file_metadata_from(
  url: &str,
  agent: &ureq::Agent,
  files: &mut Vec<CdnFileMetadata>,
) -> Result<(), anyhow::Error> {
  let update_json: UpdateJson = agent
    .get(&format!("{url}/update.json"))
    .call()
    .with_context(|| format!("Failed to load update.json from {url}"))?
    .into_json()
    .with_context(|| format!("Failed to deserialize update.json from {url}"))?;

  for (local_path, hash) in update_json.hash_list {
    files.push(CdnFileMetadata {
      local_path: local_path.clone(),
      url: format!("{url}/{local_path}"),
      hash,
    })
  }

  Ok(())
}

enum FileHashResult {
  Exists(String),
  DoesNotExist,
}

fn hash_of_local_file(local_path: &str) -> anyhow::Result<FileHashResult> {
  let mut hasher = Sha1::new();

  let exists = fs::exists(local_path)
    .with_context(|| format!("Failed to check {local_path} for existence"))?;
  if !exists {
    return Ok(FileHashResult::DoesNotExist);
  }

  let file_content =
    fs::read(local_path).with_context(|| format!("Failed to read {local_path}"))?;
  hasher.update(&file_content);
  let hash = hasher.digest().to_string();

  Ok(FileHashResult::Exists(hash))
}

fn download_files(agent: &ureq::Agent, files: Vec<CdnFileMetadata>) -> anyhow::Result<()> {
  for file in files {
    print!("Downloading {}...", file.local_path);
    io::stdout().flush().expect("must never happen");

    match download_file(agent, file) {
      Ok(_) => {
        println!("{}", "done".bright_green());
      }
      result @ Err(_) => {
        println!("{}", "failed".bright_red());
        return result;
      }
    }
  }

  Ok(())
}

fn download_file(agent: &ureq::Agent, file: CdnFileMetadata) -> anyhow::Result<()> {
  let local_path_buf = PathBuf::from(&file.local_path);
  if let Some(parent) = local_path_buf.parent() {
    fs::create_dir_all(parent)
      .with_context(|| format!("Failed to create directory for: {}", file.local_path))?;
  }

  let mut res = agent
    .get(&file.url)
    .call()
    .with_context(|| format!("Failed to download: {}", file.url))?
    .into_reader();

  let mut file_obj = fs::File::create(&file.local_path)
    .with_context(|| format!("Failed to create {}", file.local_path))?;

  io::copy(&mut res, &mut file_obj)
    .with_context(|| format!("Failed to save {}", file.local_path))?;
  Ok(())
}
