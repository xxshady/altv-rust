use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Injects to resource "main" function
///
/// altv core as first parameter and removes any other parameter from the signature
/// `fn main(core: *mut alt::__alt_ICore, full_main_path: std::path::PathBuf) -> alt::MainResource { ... }`
///
/// and `alt::__set_alt_core(core);` to the start of function body
///
/// and creates resource instance at the end of function body
/// `alt::MainResource::new(...)`
#[proc_macro_attribute]
pub fn resource_main_func(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemFn);
    let ItemFn {
        attrs,
        vis,
        sig: _sig,
        block,
    } = input;
    let statements = &block.stmts;

    quote! {
        #(#attrs)* #vis fn main(
            core: usize,
            full_main_path: String,
            __on_resource_impl_create: fn(resource: alt::__ResourceImpl)
        ) {
            unsafe { alt::__set_alt_core(core as *mut alt::__alt_ICore) };
            __on_resource_impl_create(alt::__init(full_main_path));

            #(#statements)*
        }
    }
    .into()
}
