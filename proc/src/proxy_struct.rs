use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, Attribute, ItemStruct, Type, Ident, Meta, ItemEnum};
use syn::spanned::Spanned;
use crate::utils::{append_to_tokens, get_ident_optional, get_type_ty_or, parse_metas, separate_attr_by_name};

pub(crate) fn proxy_struct(attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let metas = parse_metas(attrs);

    let struct_input = parse_macro_input!(input as ItemStruct);

    let out_quotes = temp_struct(&metas, &struct_input);

    proc_macro::TokenStream::from(quote! { #out_quotes })
}

macro_rules! map_meta_to_local {
    ($from:expr => { $($name:expr => $local:ident),* $(,)? }) => {
        let metas = ($from);
        $(let $local = $crate::utils::get_meta_by_name(metas, $name);)*
    };
}

const ATTR_PROXY_STRUCT: &str = "proxy_struct";

const ATTR_INCLUDES: &[&str] = &[
    ATTR_PROXY_STRUCT,
];
const META_ORIGIN_TYPE: &str = "origin_type";
const META_FIELD_NAME: &str = "field_name";
const META_SKIP_FORWARD: &str = "skip_forward";
const META_SKIP_BACKWARD: &str = "skip_backward";

struct TempStruct {
    pub input: ItemStruct,
    pub reserved_attrs: Vec<Attribute>,
    pub origin_type: Type,
    pub field_name: Option<Ident>,
    pub skip_forward: bool,
    pub skip_backward: bool,
}

fn temp_struct(metas: &Vec<Meta>, item_struct: &ItemStruct) -> TempStruct {
    let ItemStruct { attrs, ident, .. } = item_struct;

    let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);
    if matches!(matched.len(), n if n > 0) {
        panic!("so many proxy_struct");
    }

    map_meta_to_local!(&metas => {
        META_ORIGIN_TYPE => origin_type,
        META_FIELD_NAME => field_name,
        META_SKIP_FORWARD => skip_forward,
        META_SKIP_BACKWARD => skip_backward,
    });

    TempStruct {
        input: item_struct.clone(),
        reserved_attrs: surplus,
        origin_type: get_type_ty_or(&origin_type, &format_ident!("Origin{}", ident)),
        field_name: get_ident_optional(&field_name),
        skip_forward: skip_forward.is_some(),
        skip_backward: skip_backward.is_some(),
    }
}

impl ToTokens for TempStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { input, reserved_attrs, origin_type, field_name, skip_forward, skip_backward } = self;

        let ItemStruct { ident, vis, .. } = input;

        let mut napi_metas = Vec::new();
        if *skip_forward { napi_metas.push(quote! {object_to_js = false}) }
        if *skip_backward { napi_metas.push(quote! {object_from_js = false}) }

        let struct_body = match &field_name {
            Some(ident) => quote! { { pub(crate) #ident: #origin_type } },
            None => quote! { (pub(crate) #origin_type); },
        };

        append_to_tokens(tokens, quote! {
            #[napi( #( #napi_metas ),* )]
            #( #reserved_attrs )*
            #vis struct #ident #struct_body
        });

        if !*skip_forward {
            let from_code = match &field_name {
                Some(field) => quote! { #ident { #field: value } },
                None => quote! { #ident (value) },
            };

            append_to_tokens(tokens, quote! {
                impl From<#origin_type> for #ident {
                    fn from(value: #origin_type) -> Self {
                        #from_code
                    }
                }
            });
        }

        if !*skip_backward {
            let into_code = match &field_name {
                Some(ident) => quote! { self.#ident },
                None => quote! { self.0 },
            };

            append_to_tokens(tokens, quote! {
                impl Into<#origin_type> for #ident {
                    fn into(self) -> #origin_type {
                        #into_code
                    }
                }
            });
        }
    }
}