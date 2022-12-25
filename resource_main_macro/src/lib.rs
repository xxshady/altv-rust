use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Injects to resource "main" function
///
/// altv core as first parameter and removes any other parameter from the signature
/// `fn main(core: *mut alt::__alt_ICore) { ... }`
///
/// and `alt::__set_alt_core(core);` to the start of function body
#[proc_macro_attribute]
pub fn resource_main_func(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemFn);
    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    } = input;
    let statements = &block.stmts;

    sig.inputs.clear();
    sig.inputs
        .push(syn::parse(quote! { core: *mut alt::__alt_ICore }.into()).unwrap());
    sig.inputs
        .push(syn::parse(quote! { full_main_path: std::path::PathBuf }.into()).unwrap());

    quote! {
        #(#attrs)* #vis #sig {
            unsafe { alt::__set_alt_core(core) };
            #(#statements)*
        }
    }
    .into()
}
