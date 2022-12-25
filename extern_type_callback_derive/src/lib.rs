use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(ExternTypeCallback)]
pub fn extern_type_callback_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_extern_type(&ast)
}

fn impl_extern_type(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let type_id = format!("alt_rs::{name}");
    let gen = quote! {
        unsafe impl ExternType for #name {
            type Id = type_id!(#type_id);
            type Kind = cxx::kind::Trivial;
        }
    };
    gen.into()
}
