use crate::{
    conf_convert::{parse_conf_convert, ConfConvert, NormalConfConvert},
    utils::{append_to_tokens, get_ident_optional, get_type_ty_or, parse_metas, separate_attr_by_name},
};
use macros::define_const_str;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Attribute, Ident, ItemStruct, Meta, Type};

pub(crate) fn proxy_wrap(attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let metas = parse_metas(attrs);

    let macro_input = parse_macro_input!(input as ItemStruct);

    let proxy_wrap = parse_proxy_wrap(&metas, &macro_input);

    proc_macro::TokenStream::from(quote! { #proxy_wrap })
}

macro_rules! map_meta_to_local {
    ($from:expr => { $($name:expr => $local:ident),* $(,)? }) => {
        let metas = ($from);
        $(let $local = $crate::utils::get_meta_by_name(metas, $name);)*
    };
}

define_const_str!(ATTR_PROXY_WRAP = proxy_wrap);
const ATTR_INCLUDES: &[&str] = &[ATTR_PROXY_WRAP];

define_const_str!(
    META_ORIGIN_TYPE = origin_type,
    META_FIELD_NAME = field_name,
);

struct ProxyWrap {
    pub input: ItemStruct,
    pub reserved_attrs: Vec<Attribute>,
    pub origin_type: Type,
    pub field_name: Option<Ident>,
    pub conf_convert: ConfConvert,
}

fn parse_proxy_wrap(metas: &Vec<Meta>, item_struct: &ItemStruct) -> ProxyWrap {
    let ItemStruct { attrs, ident, .. } = item_struct;

    let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);
    if matches!(matched.len(), n if n > 0) {
        panic!("so many proxy_wrap");
    }

    map_meta_to_local!(&metas => {
        META_ORIGIN_TYPE => origin_type,
        META_FIELD_NAME => field_name,
    });

    ProxyWrap {
        input: item_struct.clone(),
        reserved_attrs: surplus,
        origin_type: get_type_ty_or(&origin_type, &format_ident!("Origin{}", ident)),
        field_name: get_ident_optional(&field_name),
        conf_convert: parse_conf_convert(metas),
    }
}

impl ToTokens for ProxyWrap {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            input, reserved_attrs, origin_type, field_name,
            conf_convert,
        } = self;

        let ItemStruct { ident, vis, .. } = input;
        let NormalConfConvert { skip_from_origin, skip_into_origin, skip_to_js, skip_from_js } = conf_convert.normal();

        let mut napi_metas = Vec::new();
        if skip_to_js { napi_metas.push(quote! {object_to_js = false}) }
        if skip_from_js { napi_metas.push(quote! {object_from_js = false}) }

        let wrap_body = match &field_name {
            Some(ident) => quote! { { pub(crate) #ident: #origin_type } },
            None => quote! { (pub(crate) #origin_type); },
        };

        append_to_tokens(tokens, quote! {
            #[napi( #( #napi_metas ),* )]
            #( #reserved_attrs )*
            #vis struct #ident #wrap_body
        });

        if !skip_from_origin {
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

        if !skip_into_origin {
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