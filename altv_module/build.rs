fn main() {
  relib_interface::host::generate(
    core_shared::EXPORTS,
    "shared::exports::Exports",
    core_shared::IMPORTS,
    "shared::imports::Imports",
  );
}
