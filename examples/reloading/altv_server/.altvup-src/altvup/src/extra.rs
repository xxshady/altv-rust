use std::fs;

use anyhow::Context;

const SERVER_CONFIG: &str = "server.toml";

pub fn create_server_config(load_jsv2: bool) -> anyhow::Result<()> {
  let exists = fs::exists(SERVER_CONFIG)
    .with_context(|| format!("Failed to check whether {SERVER_CONFIG} exists or not"))?;
  if exists {
    return Ok(());
  }

  println!("Creating {SERVER_CONFIG} with a minimum of parameters");

  let modules_param = if load_jsv2 {
    "modules = [\n  \
      \"rust-module\",\n  \
      \"js-module-v2\",\n\
    ]"
  } else {
    "modules = [\"rust-module\"]"
  };

  fs::write(
    SERVER_CONFIG,
    format!(
      "{modules_param}\n\
      resources = []\n"
    ),
  )
  .with_context(|| format!("Failed to create {SERVER_CONFIG}"))?;

  Ok(())
}
