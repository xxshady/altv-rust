use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse::Parser, ItemFn};

// copy-pasted from tokio
type AttributeArgs = syn::punctuated::Punctuated<syn::NestedMeta, syn::Token![,]>;

/// Converts the function to be compatible with the altv module.
///
/// ## `crate_name`
/// This attribute can be used if crate is renamed in Cargo.toml using "package" option.
/// ```rust
///
/// #[altvxx::main(crate_name = "altvxx")]
/// pub fn main() {}
/// ```
#[proc_macro_attribute]
pub fn resource_main_func(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as ItemFn);
    let ItemFn {
        attrs,
        vis,
        sig: _sig,
        block,
    } = input;
    let statements = &block.stmts;

    let args = AttributeArgs::parse_terminated.parse(args).unwrap();

    let mut crate_name = String::from("altvx");

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
        #(#attrs)* #vis fn main(
            core: usize, // workaround for the clippy unsafety error
            resource_handlers: &mut #crate_name_ident::__internal::ResourceHandlers,
        ) {
            unsafe { #crate_name_ident::__internal::set_alt_core(core as *mut #crate_name_ident::__internal::ICore) };
            #crate_name_ident::__internal::init(resource_handlers);

            #(#statements)*
        }
    }
    .into()
}
