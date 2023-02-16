use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

/// Injects __set_alt_core to resource "main" function
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
            core: usize, // workaround for the clippy unsafety error
            full_main_path: String,
            __on_resource_impl_create: fn(resource: &std::thread::LocalKey<std::cell::RefCell<alt::__ResourceImpl>>)
        ) {
            unsafe { alt::__set_alt_core(core as *mut alt::__alt_ICore) };
            __on_resource_impl_create(alt::__init(full_main_path));

            #(#statements)*
        }
    }
    .into()
}
