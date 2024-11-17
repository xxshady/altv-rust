use colored::*;
use anyhow::bail;
use shared::find_cli_param;

const VALID_BRANCHES: &[&str] = &[
  "release",
  "rc",
  #[cfg(debug_assertions)]
  "test-release",
];

mod shared;
mod rust_module;
mod altv_server_files;
mod extra;

fn main() {
  #[cfg(windows)]
  control::set_virtual_terminal(true).expect("must never happen");

  print_altvup();

  if let Err(e) = altvup() {
    println!("{} {e:#}", "Error:".bright_red());
  }

  print_altvup();
}

fn altvup() -> anyhow::Result<()> {
  // skipping "cargo altvup" part
  let args = std::env::args().skip(2);

  let args = args.collect::<Box<[String]>>();
  let args = &args[..];

  let [branch, other_args @ ..] = args else {
    bail!(
      "Expected branch, for example: `cargo altvup {}`",
      VALID_BRANCHES[0]
    );
  };

  if !VALID_BRANCHES.contains(&branch.as_str()) {
    bail!(
      "Invalid branch: {branch}, use one of these: {}",
      VALID_BRANCHES.join(", ")
    );
  }

  let agent = ureq::AgentBuilder::new().user_agent("cargo-altvup").build();

  println!(
    "Selected {} branch: {}",
    "alt:V".bright_green(),
    branch.bright_purple()
  );

  let load_jsv2 = find_cli_param(other_args, "jsv2").is_some();

  rust_module::compile(&agent, branch, other_args)?;
  altv_server_files::download(&agent, branch, load_jsv2, other_args)?;
  extra::create_server_config(load_jsv2)?;

  Ok(())
}

fn print_altvup() {
  println!(
    "====== {}{} ======",
    "altv".bright_green(),
    "up".bright_cyan()
  );
}
