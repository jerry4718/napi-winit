use crate::conf_convert::{parse_conf_convert, ConfConvert, NormalConfConvert};
use crate::conf_fields::{parse_conf_fields, quote_conf_fields, quote_conv_usage, ConfField, ConfFields, Kind, With};
use crate::utils::{append_to_tokens, get_type_ty_or, parse_metas, separate_attr_by_name};
use macros::{define_const_str, map_meta_to_local};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Attribute, ItemStruct, Meta, Type, Variant};

pub(crate) fn proxy_struct(attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let metas = parse_metas(attrs);

    let item_struct = parse_macro_input!(input as ItemStruct);

    let proxy_struct = parse_proxy_struct(&metas, &item_struct);

    proc_macro::TokenStream::from(quote! { #proxy_struct })
}

define_const_str!(ATTR_PROXY_STRUCT = proxy_struct);
const ATTR_INCLUDES: &[&str] = &[ATTR_PROXY_STRUCT];

define_const_str!(
    META_OBJECT = object,
    META_ORIGIN_TYPE = origin_type,
);

struct ProxyStruct {
    pub input: ItemStruct,
    pub reserved_attrs: Vec<Attribute>,
    pub origin_type: Type,
    pub conf_fields: ConfFields,
    pub conf_convert: ConfConvert,
    pub object: bool,
}

fn parse_proxy_struct(metas: &Vec<Meta>, item_struct: &ItemStruct) -> ProxyStruct {
    let ItemStruct { attrs, ident, fields, .. } = item_struct;

    let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);
    if matches!(matched.len(), n if n > 0) {
        panic!("so many proxy_struct");
    }

    map_meta_to_local!(&metas => {
        META_OBJECT => object,
        META_ORIGIN_TYPE => origin_type,
    });

    ProxyStruct {
        input: item_struct.clone(),
        reserved_attrs: surplus,
        conf_fields: parse_conf_fields(fields, &matched, ATTR_PROXY_STRUCT, ATTR_INCLUDES),
        origin_type: get_type_ty_or(&origin_type, &format_ident!("Origin{}", ident)),
        conf_convert: parse_conf_convert(metas),
        object: object.is_some(),
    }
}

impl ToTokens for ProxyStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            input, reserved_attrs, origin_type, conf_fields,
            conf_convert, object,
        } = self;

        let ItemStruct { ident, vis, .. } = input;
        let ConfFields { fields, .. } = conf_fields;
        let NormalConfConvert { skip_from_origin, skip_into_origin, skip_to_js, skip_from_js } = conf_convert.normal();

        let mut napi_metas = Vec::new();
        if *object { napi_metas.push(quote! {object}) }
        if skip_to_js { napi_metas.push(quote! {object_to_js = false}) }
        if skip_from_js { napi_metas.push(quote! {object_from_js = false}) }

        append_to_tokens(tokens, quote! {
            #[napi( #( #napi_metas ),* )]
            #( #reserved_attrs )*
            #vis struct #ident {
                #( #fields ),*
            }
        });

        if !skip_from_origin {
            append_to_tokens(tokens, quote_from_origin(ident, &origin_type, &conf_fields));
        }

        if !skip_into_origin {
            append_to_tokens(tokens, quote_into_origin(ident, &origin_type, &conf_fields));
        }
    }
}

fn quote_from_origin(ident: &Ident, origin_type: &Type, conf_fields: &ConfFields) -> TokenStream {
    let dispose = quote_conf_fields(
        conf_fields, Kind::Dispose, With::Origin,
        |ConfField { ident, .. }| quote! { #ident },
    );
    let compose = quote_conf_fields(
        conf_fields, Kind::Compose, With::Proxy,
        |ConfField { ident, from_origin, .. }| quote_conv_usage(ident, from_origin),
    );
    quote! {
        impl From<#origin_type> for #ident {
            fn from(value: #origin_type) -> Self {
                let #origin_type #dispose = value;
                Self #compose
            }
        }
    }
}

fn quote_into_origin(ident: &Ident, origin_type: &Type, conf_fields: &ConfFields) -> TokenStream {
    let dispose = quote_conf_fields(
        conf_fields, Kind::Dispose, With::Proxy,
        |ConfField { ident, .. }| quote! { #ident },
    );
    let compose = quote_conf_fields(
        conf_fields, Kind::Compose, With::Origin,
        |ConfField { ident, into_origin, .. }| quote_conv_usage(ident, into_origin),
    );
    quote! {
        impl Into<#origin_type> for #ident {
            fn into(self) -> #origin_type {
                let Self #dispose = self;
                #origin_type #compose
            }
        }
    }
}
