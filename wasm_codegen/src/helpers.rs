use proc_macro::TokenStream;
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};

use crate::{
    parser,
    value_type::{ValueRepr, ValueType},
};

pub(crate) struct Params {
    interface_file: String,
}

impl Parse for Params {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let file_path = input.parse::<syn::LitStr>().expect("Expected file path");
        Ok(Params {
            interface_file: file_path.value(),
        })
    }
}

pub(crate) fn parse_interface_file(input: TokenStream) -> (parser::Interface, String) {
    let params = syn::parse::<Params>(input).unwrap();
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let interface_file = manifest_dir.join(params.interface_file);
    let interface_file = interface_file.to_string_lossy().to_string();
    (parser::parse_interface(&interface_file), interface_file)
}

pub(crate) fn build_code(
    code: proc_macro2::TokenStream,
    interface_file: String,
) -> proc_macro2::TokenStream {
    let mut code = code.to_string();

    // this is needed for rustc to rebuild source code if interface file changed
    code += &format!("const _: &str = include_str!(r#{interface_file:?}#);\n\n");

    code.parse().unwrap()
}

pub(crate) fn value_type_to_repr_as_token_stream(
    value_type: ValueType,
) -> proc_macro2::TokenStream {
    let repr: ValueRepr = value_type.into();
    let repr_str: &str = repr.into();
    repr_str.parse().unwrap()
}

pub(crate) fn value_type_to_rust_as_syn_type(
    value_type: ValueType,
    deserialization: bool,
) -> syn::Type {
    syn::parse_str(if deserialization {
        value_type.de().unwrap_or(value_type.rust())
    } else {
        value_type.rust()
    })
    .expect("value_type_to_rust_type_as_token_stream failed")
}
