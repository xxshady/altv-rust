use crate::{parser, value_type::ValueKind};
use proc_macro::TokenStream;
use quote::quote;
use syn::Ident;

use crate::helpers::{
    build_code, parse_interface_file, value_type_to_repr_as_token_stream,
    value_type_to_rust_as_syn_type,
};

pub(crate) fn gen_exports(input: TokenStream) -> proc_macro2::TokenStream {
    let (parser::Interface { exports, .. }, interface_file) = parse_interface_file(input);

    let mut private_props = vec![];
    let mut private_prop_names = vec![];
    let mut exported_names = vec![];
    let mut pub_methods = vec![];

    for parser::Func { name, params, ret } in exports {
        let name: Ident = syn::parse_str(&name).unwrap();
        let name_call: Ident = syn::parse_str(&format!("call_{name}")).unwrap();
        let name_prop: Ident = syn::parse_str(&format!("prop_{name}")).unwrap();
        let exported_name: Ident = syn::parse_str(&format!("__custom_exports_{name}")).unwrap();

        let mut param_names = vec![];
        let mut params_signature = vec![];
        let mut params_prop_types = vec![];
        let mut params_serialization = vec![];

        for parser::Param { name, param_type } in params {
            let name: Ident = syn::parse_str(&name).unwrap();
            let internal_type = value_type_to_repr_as_token_stream(param_type);
            let serialization = match param_type.kind() {
                ValueKind::Native => quote! { #name as #internal_type },
                ValueKind::FatPtr => quote! { self.send_to_guest(&#name)? },
                ValueKind::Bool => quote! { #name as i32 },
            };
            let param_type = value_type_to_rust_as_syn_type(param_type);

            param_names.push(name.clone());
            params_signature.push(quote! {
                #name: #param_type
            });
            params_prop_types.push(internal_type);
            params_serialization.push(serialization);
        }

        let (ret_type, ret_prop_type, ret_deserialization) = if let Some(ret_type) = ret {
            let pub_type = value_type_to_rust_as_syn_type(ret_type);
            let internal_type = value_type_to_repr_as_token_stream(ret_type);
            let deserialization = match ret_type.kind() {
                ValueKind::Native => quote! { Ok(call_return as #pub_type) },
                ValueKind::FatPtr => quote! { self.read_from_guest(call_return) },
                ValueKind::Bool => quote! { Ok(call_return == 1) },
            };

            (quote! { #pub_type }, internal_type, deserialization)
        } else {
            (quote! { () }, quote! { () }, quote! { Ok(()) })
        };

        private_props.push(quote! {
            #name_prop: wasmtime::TypedFunc<( #( #params_prop_types, )* ), #ret_prop_type>
        });

        private_prop_names.push(name_prop.clone());
        exported_names.push(exported_name);

        pub_methods.push(quote! {
            pub fn #name_call(&mut self, #( #params_signature, )* ) -> wasmtime::Result<#ret_type> {
                #[allow(clippy::unnecessary_cast)]
                {
                    #(
                        let #param_names = #params_serialization;
                    )*

                    #[allow(unused_variables, clippy::let_unit_value)]
                    let call_return = self.#name_prop.call(
                        &mut self.store,
                        ( #( #param_names, )* )
                    )?;
                    #ret_deserialization
                }
            }
        });
    }

    build_code(
        quote! {
            pub mod exports {
                pub struct Exports<S> {
                    #( #private_props, )*

                    memory: wasmtime::Memory,
                    alloc: wasmtime::TypedFunc<super::__shared::Size, super::__shared::Ptr>,
                    free: wasmtime::TypedFunc<super::__shared::FatPtr, ()>,
                    store: wasmtime::Store<S>,
                }

                impl<S> Exports<S> {
                    pub fn new(mut store: wasmtime::Store<S>, instance: wasmtime::Instance) -> Self {
                        Self {
                            // user funcs
                            #(
                                #private_prop_names: instance.get_typed_func(&mut store, stringify!(#exported_names)).unwrap(),
                            )*

                            // internal
                            memory: instance.get_memory(&mut store, "memory").unwrap(),
                            alloc: instance.get_typed_func(&mut store, "__custom_alloc").unwrap(),
                            free: instance.get_typed_func(&mut store, "__custom_free").unwrap(),
                            store,
                        }
                    }

                    fn send_to_guest<T: ?Sized + serde::Serialize>(
                        &mut self,
                        data: &T,
                    ) -> wasmtime::Result<super::__shared::FatPtr> {
                        let encoded = bincode::serialize(&data)?;
                        let size = encoded.len().try_into()?;
                        let ptr = self.alloc.call(&mut self.store, size)?;
                        self.memory.write(&mut self.store, ptr as usize, &encoded)?;
                        Ok(super::__shared::to_fat_ptr(ptr, size))
                    }

                    fn read_from_guest<T: serde::de::DeserializeOwned>(
                        &mut self,
                        fat_ptr: super::__shared::FatPtr,
                    ) -> wasmtime::Result<T> {
                        let (ptr, size) = super::__shared::from_fat_ptr(fat_ptr);
                        let mut buffer = vec![0; size as usize];
                        self.memory
                            .read(&mut self.store, ptr as usize, &mut buffer)?;
                        let decoded: T = bincode::deserialize(&buffer)?;

                        self.free.call(&mut self.store, fat_ptr).unwrap();
                        Ok(decoded)
                    }

                    #( #pub_methods )*
                }
            }
        },
        interface_file,
    )
}

pub(crate) fn impl_imports(input: TokenStream) -> proc_macro2::TokenStream {
    let (parser::Interface { imports, .. }, interface_file) = parse_interface_file(input);

    let mut trait_methods = vec![];
    let mut linker_funcs = vec![];

    for parser::Func { name, params, ret } in imports {
        let name: Ident = syn::parse_str(&name).unwrap();

        let mut param_trait_decls = vec![];
        let mut param_internal_decls = vec![];
        let mut param_names = vec![];
        let mut param_deserializations = vec![];

        for p in params {
            // TODO: make proper panic messages
            let name: Ident = syn::parse_str(&p.name).expect("t");
            let pub_type = value_type_to_rust_as_syn_type(p.param_type);
            let param_internal_type = value_type_to_repr_as_token_stream(p.param_type);
            let deserialization = match p.param_type.kind() {
                ValueKind::Native => quote! { #name as #pub_type },
                ValueKind::FatPtr => {
                    quote! { read_from_guest(&mut caller, #name).unwrap() }
                }
                ValueKind::Bool => {
                    quote! { #name == 1 }
                }
            };

            param_trait_decls.push(quote! {
                #name: #pub_type
            });
            param_internal_decls.push(quote! {
                #name: #param_internal_type
            });
            param_deserializations.push(quote! {
                #deserialization
            });
            param_names.push(name);
        }

        let (pub_ret, internal_ret, ret_serialization) = if let Some(ret_type) = ret {
            let internal_type = value_type_to_repr_as_token_stream(ret_type);
            let serialization = match ret_type.kind() {
                ValueKind::Native => quote! { call_return as #internal_type },
                ValueKind::FatPtr => {
                    quote! { send_to_guest(&mut caller, &call_return).unwrap() }
                }
                ValueKind::Bool => quote! { call_return as i32 },
            };

            let ret_type = value_type_to_rust_as_syn_type(ret_type);

            (
                quote! { -> #ret_type },
                quote! { -> #internal_type },
                serialization,
            )
        } else {
            (quote! {}, quote! {}, quote! {})
        };

        trait_methods.push(quote! {
            fn #name(&self, #( #param_trait_decls, )* ) #pub_ret;
        });

        linker_funcs.push(quote! {
            linker
                .func_wrap(
                    "__custom_imports",
                    stringify!(#name),
                    #[allow(unused_mut)]
                    |mut caller: wasmtime::Caller<U>, #( #param_internal_decls, )*| #internal_ret {
                        #[allow(clippy::unnecessary_cast)]
                        {
                            #(
                                let #param_names = #param_deserializations;
                            )*
                            #[allow(unused_variables, clippy::let_unit_value)]
                            let call_return = caller.data().#name( #( #param_names, )* );
                            #ret_serialization
                        }
                    },
                )
                .unwrap();
        });
    }

    build_code(
        quote! {
            pub mod imports {
                pub trait Imports {
                    #( #trait_methods )*
                }

                pub fn add_to_linker<U: Imports>(linker: &mut wasmtime::Linker<U>) {
                    fn get_memory_and<U: Imports, Params: wasmtime::WasmParams, Results: wasmtime::WasmResults>(
                        caller: &mut wasmtime::Caller<U>,
                        and: &'static str,
                    ) -> (wasmtime::Memory, wasmtime::TypedFunc<Params, Results>) {
                        let Some(wasmtime::Extern::Memory(memory)) = caller.get_export("memory") else {
                            panic!("Failed to get memory export")
                        };
                        let Some(wasmtime::Extern::Func(func)) = caller.get_export(and) else {
                            panic!("Failed to get {and:?} export")
                        };

                        (memory, func.typed::<Params, Results>(caller).unwrap())
                    }

                    fn read_from_guest<U: Imports, T: serde::de::DeserializeOwned>(
                        mut caller: &mut wasmtime::Caller<U>,
                        fat_ptr: super::__shared::FatPtr,
                    ) -> wasmtime::Result<T> {
                        let (memory, free) = get_memory_and::<U, super::__shared::FatPtr, ()>(caller, "__custom_free");

                        let (ptr, size) = super::__shared::from_fat_ptr(fat_ptr);
                        let mut buffer = vec![0; size as usize];
                        memory.read(&mut caller, ptr as usize, &mut buffer)?;
                        let decoded: T = bincode::deserialize(&buffer)?;

                        free.call(&mut caller, fat_ptr)?;
                        Ok(decoded)
                    }

                    fn send_to_guest<U: Imports, T: ?Sized + serde::Serialize>(
                        mut caller: &mut wasmtime::Caller<U>,
                        data: &T,
                    ) -> wasmtime::Result<super::__shared::FatPtr> {
                        let (memory, alloc) = get_memory_and::<U, super::__shared::Size, super::__shared::Ptr>(caller, "__custom_alloc");

                        let encoded = bincode::serialize(&data)?;
                        let size = encoded.len().try_into()?;

                        let ptr = alloc.call(&mut caller, size)?;
                        memory.write(&mut caller, ptr as usize, &encoded)?;

                        Ok(super::__shared::to_fat_ptr(ptr, size))
                    }

                    #( #linker_funcs )*
                }
            }
        },
        interface_file,
    )
}
