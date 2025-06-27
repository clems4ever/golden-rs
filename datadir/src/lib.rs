extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Lit, Meta, NestedMeta};

#[proc_macro_attribute]
pub fn async_datadir(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let function = parse_macro_input!(input as ItemFn);
    let block = &function.block;
    let inputs = &function.sig.inputs;
    let output = &function.sig.output;
    let func_name = &function.sig.ident;
    let attrs = &function.attrs;

    let prefix = args
        .iter()
        .map(|arg| {
            if let NestedMeta::Meta(Meta::NameValue(meta)) = arg {
                if meta.path.is_ident("base_path") {
                    if let Lit::Str(lit_str) = &meta.lit {
                        return Some(lit_str.value());
                    }
                }
            }
            Some(".".to_string())
        })
        .next()
        .unwrap_or_else(|| panic!("Expected a named string literal with the key 'base_path'"));

    let output = quote! {
        #(#attrs)*
        async fn #func_name() #output {
            use std::path::{Path};
            let datadir = Path::new(#prefix).join(module_path!()).join(stringify!(#func_name));
            async fn _wrapped(#inputs) #output #block
            _wrapped(&datadir).await
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn datadir(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let function = parse_macro_input!(input as ItemFn);
    let block = &function.block;
    let inputs = &function.sig.inputs;
    let output = &function.sig.output;
    let func_name = &function.sig.ident;
    let attrs = &function.attrs;

    let prefix = args
        .iter()
        .map(|arg| {
            if let NestedMeta::Meta(Meta::NameValue(meta)) = arg {
                if meta.path.is_ident("base_path") {
                    if let Lit::Str(lit_str) = &meta.lit {
                        return Some(lit_str.value());
                    }
                }
            }
            Some(".".to_string())
        })
        .next()
        .unwrap_or_else(|| panic!("Expected a named string literal with the key 'base_path'"));

    let output = quote! {
        #(#attrs)*
        fn #func_name() #output {
            use std::path::{Path};
            let datadir = Path::new(#prefix).join(module_path!()).join(stringify!(#func_name));
            fn _wrapped(#inputs) #output #block
            _wrapped(&datadir)
        }
    };

    output.into()
}
