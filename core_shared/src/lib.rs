pub mod abi_stable;
pub mod result;

pub mod exports;
pub mod imports;

pub const EXPORTS: &str = include_str!("exports.rs");
pub const IMPORTS: &str = include_str!("imports.rs");

pub type ResourceName = String;
pub type ResourceNameRef<'a> = &'a str;
