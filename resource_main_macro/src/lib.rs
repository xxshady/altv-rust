#![allow(clippy::needless_doctest_main)]

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parser, ItemFn};

// copy-pasted from tokio
type AttributeArgs = syn::punctuated::Punctuated<syn::NestedMeta, syn::Token![,]>;

/// Converts the main function of your alt:V Rust resource
/// for compatibility with the alt:V module.
///
/// ## Example
/// ```
/// #[altv::main]
/// fn main() {
///    altv::log!("hello world");
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
/// ```rust
/// #[my_custom_name::main(crate_name = "my_custom_name")]
/// fn main() {
///    my_custom_name::log!("hello world");
/// }
/// ```
#[proc_macro_attribute]
pub fn resource_main_func(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemFn);
    let ItemFn {
        attrs,
        vis: _,
        sig: _sig,
        block,
    } = input;
    let statements = &block.stmts;

    let args = AttributeArgs::parse_terminated.parse(args).unwrap();

    let mut crate_name = String::from("altv");

    for arg in args {
        match arg {
            syn::NestedMeta::Meta(syn::Meta::NameValue(name_value)) => {
                let ident = name_value
                    .path
                    .get_ident()
                    .expect(".path.get_ident()")
                    .to_string()
                    .to_lowercase();
                match ident.as_str() {
                    "crate_name" => {
                        if let syn::Lit::Str(s) = name_value.lit {
                            let path = s.parse::<syn::Path>().expect("s.parse::<syn::Path>()");
                            let ident = path
                                .get_ident()
                                .cloned()
                                .expect("path.get_ident().cloned()");
                            crate_name = ident.to_string();
                        } else {
                            panic!("crate attr is not syn::Lit::Str");
                        }
                    }
                    ident => panic!("unknown arg: {ident}"),
                }
            }
            _ => panic!("invalid arg"),
        }
    }

    let crate_name_ident = syn::Ident::new(&crate_name, Span::call_site());

    quote! {
        #[no_mangle]
        #(#attrs)* extern "C" fn main(
            core: usize, // workaround for the clippy unsafety error
            resource_name: String,
            resource_handlers: &mut #crate_name_ident::__internal::ResourceHandlers,
            module_handlers: #crate_name_ident::__internal::ModuleHandlers,
        ) {
            unsafe { #crate_name_ident::__internal::set_alt_core(core as *mut #crate_name_ident::__internal::ICore) };
            #crate_name_ident::__internal::init(resource_name, resource_handlers, module_handlers);

            #(#statements)*
        }
    }
    .into()
}
