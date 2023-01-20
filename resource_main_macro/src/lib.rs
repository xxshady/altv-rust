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
/// `alt::MainResource::new(full_main_path)`
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

    // sig.inputs.clear();
    // sig.inputs
    //     .push(syn::parse(quote! { core: *mut alt::__alt_ICore }.into()).unwrap());
    // sig.inputs
    //     .push(syn::parse(quote! { full_main_path: std::path::PathBuf }.into()).unwrap());
    // sig.inputs.push(
    //     syn::parse(
    //         quote! { resource_api: std::sync::Arc<std::sync::Mutex<alt::__resource_api::ResourceApi>> }.into(),
    //     )
    //     .unwrap(),
    // );

    // sig.output = syn::parse(quote! { -> alt::MainResource }.into()).unwrap();

    quote! {
        #(#attrs)* #vis fn main(
            core: usize,
            full_main_path: std::path::PathBuf,
            resource_api: std::sync::Arc<std::sync::Mutex<alt::__resource_api::ResourceApi>>
        ) -> alt::MainResource {
            unsafe { alt::__set_alt_core(core as *mut alt::__alt_ICore) };
            // TODO: improve this shit
            let __alt_main_resource_instance = alt::MainResource::new(full_main_path, resource_api);
            #(#statements)*;
            __alt_main_resource_instance
        }
    }
    .into()
}
