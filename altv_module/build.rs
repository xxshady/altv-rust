fn main() {
  relib_interface::host::generate(
    core_shared::EXPORTS,
    "core_shared::exports::Exports",
    core_shared::IMPORTS,
    "core_shared::imports::Imports",
  );
}
