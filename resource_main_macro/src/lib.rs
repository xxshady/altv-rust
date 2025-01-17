use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, spanned::Spanned, ItemFn};

/// Converts the main function of your alt:V Rust resource
/// for compatibility with the alt:V module.
///
/// ## Example
/// ```rust,ignore
/// #[altv::main]
/// fn main() -> impl altv::IntoVoidResult {
///   altv::log!("hello world");
/// }
/// ```
///
/// ## `crate_name`
/// This attribute can be used if `altv` crate is renamed in Cargo.toml using "package" option.
///
/// Cargo.toml
/// ```toml
/// [dependencies]
/// my_custom_name = { version = "...", package = "altv" }
/// ```
///
/// src/lib.rs
/// ```rust,ignore
/// #[my_custom_name::main(crate_name = "my_custom_name")]
/// fn main() -> impl my_custom_name::IntoVoidResult {
///   my_custom_name::log!("hello world");
/// }
/// ```
#[proc_macro_attribute]
pub fn resource_main_func(params: TokenStream, input: TokenStream) -> TokenStream {
  let fn_item = {
    let input = input.clone();
    syn::parse_macro_input!(input as ItemFn)
  };
  let syn::ItemFn {
    sig: fn_sig,
    block: fn_block,
    ..
  } = fn_item;
  let fn_ident = fn_sig.ident;
  if !fn_sig.inputs.is_empty() {
    return compile_error(fn_sig.inputs, "main function can't have any arguments");
  }

  let crate_name = parse_crate_name_from_params(params);

  let wrapped_fn = quote! {
    fn #fn_ident() -> bool {
      use #crate_name::IntoVoidResult;
      fn user_code() -> impl IntoVoidResult #fn_block

      match user_code().into_void_result() {
        Ok(()) => {
          true
        }
        Err(err) => {
          #crate_name::__internal::on_main_error(err);
          false
        }
      }
    }
  };

  let exportified_fn = relib_exportify::exportify(wrapped_fn);

  quote! {
    // NOTE ⚠️: relib_module must be imported because it exports internal symbols that are required for relib_host crate
    use #crate_name::__internal::relib_module as _;

    #exportified_fn
  }
  .into()
}

fn compile_error(spanned: impl Spanned, message: &str) -> TokenStream {
  syn::Error::new(spanned.span(), message)
    .to_compile_error()
    .into()
}

fn parse_crate_name_from_params(params: TokenStream) -> syn::Ident {
  let mut crate_name = "altv".to_owned();

  let parser = syn::meta::parser(|meta| {
    assert!(
      meta.path.is_ident("crate_name"),
      "expected crate_name parameter"
    );
    let literal: syn::LitStr = meta.value()?.parse()?;
    crate_name = literal.value();
    Ok(())
  });

  parser
    .parse(params)
    .expect("Failed to parse altv::main parameters");

  quote::format_ident!("{crate_name}")
}
