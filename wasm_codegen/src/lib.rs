use proc_macro::TokenStream;
use quote::quote;

mod guest;
mod helpers;
mod host;
mod parser;
mod shared;
mod value_type;

#[proc_macro]
pub fn host(input: TokenStream) -> TokenStream {
    let exports = host::gen_exports(input.clone());
    let imports = host::impl_imports(input);
    let shared = shared::shared_mod();

    quote! {
        mod host {
            #shared

            pub type FreeFunc = wasmtime::TypedFunc<FatPtr, ()>;
            pub type AllocFunc = wasmtime::TypedFunc<Size, Ptr>;

            #exports
            #imports
        }
    }
    .into()
}

#[proc_macro]
pub fn guest(input: TokenStream) -> TokenStream {
    let helpers = guest::gen_helpers();
    let imports = guest::gen_imports(input.clone());
    let exports = guest::impl_exports(input);
    let shared = shared::shared_mod();

    quote! {
        mod guest {
            #shared
            #helpers
            #imports
            #exports
        }
    }
    .into()
}
