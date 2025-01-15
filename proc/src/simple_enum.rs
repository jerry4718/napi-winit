use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemEnum};
use crate::utils::to_lit_str;

pub fn simple_enum(input: TokenStream) -> TokenStream {
    let ItemEnum { ident, variants, .. } = parse_macro_input!(input as ItemEnum);

    let enum_ident = ident.clone();
    let enum_ident_lit = to_lit_str(Box::new(enum_ident.clone()));

    let mapped_ident = format_ident!("Origin{}", enum_ident);

    let variant_idents: Vec<_> = variants.iter().map(|v| v.ident.clone()).collect();

    TokenStream::from(quote!(
        #[napi(string_enum)]
        pub enum #enum_ident {
            #(#variant_idents,)*
        }
        impl From<#mapped_ident> for #enum_ident {
            fn from(value: #mapped_ident) -> Self {
                match value {
                    #(#mapped_ident::#variant_idents => #enum_ident::#variant_idents,)*
                    _ => todo!()
                }
            }
        }
        impl Into<#mapped_ident> for #enum_ident {
            fn into(self) -> #mapped_ident {
                match self {
                    #(#enum_ident::#variant_idents => #mapped_ident::#variant_idents,)*
                    _ => todo!()
                }
            }
        }
    ))
}