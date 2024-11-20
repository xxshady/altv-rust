pub fn find_cli_param(args: &[String], name: &str) -> Option<String> {
  let param_start = format!("--{name}");

  let param_with_value = args.iter().find(|arg| arg.starts_with(&param_start));

  match param_with_value {
    Some(param_with_value) => {
      if param_with_value.len() > param_start.len() {
        // + 1 because full syntax is: "--name=value"
        let value = &param_with_value[param_start.len() + 1..];
        Some(value.into())
      } else {
        Some(String::new())
      }
    }
    _ => None,
  }
}
