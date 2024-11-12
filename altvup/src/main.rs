use colored::*;
use anyhow::bail;

mod shared;
mod rust_module;
mod altv_server_files;

fn main() -> anyhow::Result<()> {
  #[cfg(windows)]
  control::set_virtual_terminal(true).expect("must never happen");

  print_altvup();
  altvup()?;
  print_altvup();

  Ok(())
}

fn altvup() -> anyhow::Result<()> {
  // skipping "cargo altvup" part
  let args = std::env::args().skip(2);

  let args = args.collect::<Box<[String]>>();
  let args = &args[..];

  let [branch, other_args @ ..] = args else {
    bail!("Expected branch, example usage: cargo altvup release");
  };

  let agent = ureq::AgentBuilder::new().user_agent("cargo-altvup").build();

  println!(
    "Selected {} branch: {}",
    "alt:V".bright_green(),
    branch.bright_purple()
  );

  rust_module::compile(&agent, branch, other_args)?;
  altv_server_files::download(&agent, branch)?;

  Ok(())
}

fn print_altvup() {
  println!(
    "====== {}{} ======",
    "altv".bright_green(),
    "up".bright_cyan()
  );
}
