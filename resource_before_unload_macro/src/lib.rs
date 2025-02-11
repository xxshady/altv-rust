use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, ItemFn};

// TODO: add link to reloading feature docs
/// Defines `before_unload` callback of your alt:V Rust resource.
/// It will be called when resource is unloaded when reloading feature is enabled,
/// for example using `stop <resource name>` command in the server console.
///
/// ## Example
/// ```rust,ignore
/// #[altv::before_unload]
/// fn before_unload() {
///   altv::log!("before unload");
/// }
/// ```
#[proc_macro_attribute]
pub fn resource_before_unload(_params: TokenStream, input: TokenStream) -> TokenStream {
  let fn_item = {
    let input = input.clone();
    syn::parse_macro_input!(input as ItemFn)
  };
  let syn::ItemFn { sig: fn_sig, .. } = &fn_item;
  let fn_ident = &fn_sig.ident;

  if fn_ident != "before_unload" {
    return compile_error(
      fn_ident,
      "before_unload callback must be named \"before_unload\"",
    );
  }
  if !fn_sig.inputs.is_empty() {
    return compile_error(
      &fn_sig.inputs,
      "before_unload callback can't have any arguments",
    );
  }

  let exportified_fn = relib_exportify::exportify(fn_item.into_token_stream());
  exportified_fn.into()
}

fn compile_error(spanned: impl Spanned, message: &str) -> TokenStream {
  syn::Error::new(spanned.span(), message)
    .to_compile_error()
    .into()
}
