const CDN_ADDRESS: &str = "cdn.alt-mp.com";

struct CdnFile {
  local_path: String,
  url: String,
  hash: String,
}

pub fn download(agent: &ureq::Agent, branch: &str) -> anyhow::Result<()> {
  let files = fetch_downloadable_files(branch);
  check_file_hashes()?;

  Ok(())
}

fn check_file_hashes() -> anyhow::Result<()> {
  println!("Checking file hashes");

  Ok(())
}

fn fetch_downloadable_files(branch: &str) -> anyhow::Result<Vec<CdnFile>> {
  let platform = if cfg!(target_os = "windows") {
    "x64_win32"
  } else {
    "x64_linux"
  };

  let data_files_url = format!("https://{CDN_ADDRESS}/data/${branch}/update.json");
  let server_files_url = format!("https://${CDN_ADDRESS}/server/{branch}/{platform}/update.json");
}
