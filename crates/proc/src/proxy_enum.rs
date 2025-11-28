use crate::{
    conf_convert::{parse_conf_convert, ConfConvert, NormalConfConvert},
    conf_fields::{parse_conf_fields, quote_conf_fields, ConfField, ConfFields, Kind, With},
    conf_usage::quote_option_conf_usage,
    utils::{append_to_tokens, get_type_ty_or, parse_metas, separate_attr_by_name},
};
use macros::{define_const_str, map_meta_to_local};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Attribute, ItemEnum, Meta, Type, Variant};

pub(crate) fn proxy_enum(attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let metas = parse_metas(attrs);

    let item_enum = parse_macro_input!(input as ItemEnum);

    let proxy_enum = parse_proxy_enum(&metas, &item_enum);

    proc_macro::TokenStream::from(quote! { #proxy_enum })
}

define_const_str!(ATTR_PROXY_ENUM = proxy_enum);
const ATTR_INCLUDES: &[&str] = &[ATTR_PROXY_ENUM];

define_const_str!(
    META_ORIGIN_ENUM = origin_type,
    META_STRING_ENUM = string_enum,
    META_NON_EXHAUSTIVE = non_exhaustive,
);

struct ProxyEnum {
    pub input: ItemEnum,
    pub reserved_attrs: Vec<Attribute>,
    pub proxy_variants: Vec<ProxyVariant>,
    pub origin_type: Type,
    pub string_enum: bool,
    pub non_exhaustive: bool,
    pub conf_convert: ConfConvert,
}

fn parse_proxy_enum(metas: &Vec<Meta>, item_enum: &ItemEnum) -> ProxyEnum {
    let ItemEnum { attrs, ident, variants, .. } = item_enum;

    let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);
    if matches!(matched.len(), n if n > 0) {
        panic!("so many proxy_enum");
    }

    map_meta_to_local!(&metas => {
        META_ORIGIN_ENUM => origin_type,
        META_STRING_ENUM => string_enum,
        META_NON_EXHAUSTIVE => non_exhaustive,
    });

    ProxyEnum {
        input: item_enum.clone(),
        reserved_attrs: surplus,
        proxy_variants: parse_proxy_variants(string_enum.is_some(), variants.iter().collect()),
        origin_type: get_type_ty_or(&origin_type, &format_ident!("Origin{}", ident)),
        string_enum: string_enum.is_some(),
        non_exhaustive: non_exhaustive.is_some(),
        conf_convert: parse_conf_convert(metas),
    }
}

impl ToTokens for ProxyEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            input, reserved_attrs, proxy_variants,
            origin_type, string_enum, non_exhaustive, conf_convert,
        } = self;

        let ItemEnum { ident, vis, .. } = input;
        let NormalConfConvert { skip_from_origin, skip_into_origin, skip_to_js, skip_from_js } = conf_convert.normal();

        let mut napi_metas = Vec::new();
        if *string_enum { napi_metas.push(quote! {string_enum}) }
        if skip_to_js { napi_metas.push(quote! {object_to_js = false}) }
        if skip_from_js { napi_metas.push(quote! {object_from_js = false}) }

        let non_exhaustive_variant = if !(*non_exhaustive) { vec![] } else { vec![quote! { NonExhaustive }] };

        append_to_tokens(tokens, quote_spanned! { input.span() =>
            #[napi( #( #napi_metas ),* )]
            #( #reserved_attrs )*
            #vis enum #ident {
                #( #proxy_variants, )*
                #( #non_exhaustive_variant, )*
            }
        });

        if !skip_from_origin {
            let conv = if !*string_enum { conv_structure_from } else { conv_unit_from };
            let conv_from: Vec<_> = proxy_variants
                .iter()
                .map(|variant| conv(variant, origin_type))
                .collect();

            let non_exhaustive_from = if !(*non_exhaustive) {
                vec![]
            } else {
                vec![quote! { _ => Self::NonExhaustive }]
            };

            append_to_tokens(tokens, quote! {
                impl From<#origin_type> for #ident {
                    fn from(value: #origin_type) -> Self {
                        match value {
                            #( #conv_from, )*
                            #( #non_exhaustive_from, )*
                        }
                    }
                }
            });
        }

        if !skip_into_origin {
            let conv = if !*string_enum { conv_structure_into } else { conv_unit_into };
            let conv_into: Vec<_> = proxy_variants
                .iter()
                .map(|variant| conv(variant, origin_type))
                .collect();

            let non_exhaustive_into = if !(*non_exhaustive) {
                vec![]
            } else {
                vec![quote! { Self::NonExhaustive => unreachable!(stringify!(#ident::NonExhaustive)) }]
            };

            append_to_tokens(tokens, quote! {
                impl Into<#origin_type> for #ident {
                    fn into(self) -> #origin_type {
                        match self {
                            #( #conv_into, )*
                            #( #non_exhaustive_into, )*
                        }
                    }
                }
            });
        }
    }
}

fn conv_structure_from(variant: &ProxyVariant, origin_type: &Type) -> TokenStream {
    let ProxyVariant { input: Variant { ident, .. }, conf_fields, .. } = variant;
    let dispose = quote_conf_fields(
        conf_fields, Kind::Dispose, With::Origin,
        |ConfField { field_ident, .. }| quote! { #field_ident },
    );

    let compose = quote_conf_fields(
        conf_fields, Kind::Compose, With::Proxy,
        |ConfField { field_ident, from_origin, .. }| quote_option_conf_usage(field_ident, from_origin),
    );
    quote! { #origin_type::#ident #dispose => Self::#ident #compose }
}

fn conv_structure_into(variant: &ProxyVariant, origin_type: &Type) -> TokenStream {
    let ProxyVariant { input: Variant { ident, .. }, conf_fields, .. } = variant;
    let dispose = quote_conf_fields(
        conf_fields, Kind::Dispose, With::Proxy,
        |ConfField { field_ident, .. }| quote! { #field_ident },
    );
    let compose = quote_conf_fields(
        conf_fields, Kind::Compose, With::Origin,
        |ConfField { field_ident, into_origin, .. }| quote_option_conf_usage(field_ident, into_origin),
    );
    quote! { Self::#ident #dispose => #origin_type::#ident #compose }
}

fn conv_unit_from(variant: &ProxyVariant, origin_type: &Type) -> TokenStream {
    let ProxyVariant { input: Variant { ident, .. }, .. } = variant;
    quote! { #origin_type::#ident => Self::#ident }
}

fn conv_unit_into(variant: &ProxyVariant, origin_type: &Type) -> TokenStream {
    let ProxyVariant { input: Variant { ident, .. }, .. } = variant;
    quote! { Self::#ident => #origin_type::#ident }
}

struct ProxyVariant {
    pub input: Variant,
    pub reserved_attrs: Vec<Attribute>,
    pub conf_fields: ConfFields,
    pub string_enum: bool,
}

fn parse_proxy_variants(string_enum: bool, variants: Vec<&Variant>) -> Vec<ProxyVariant> {
    variants.iter()
        .map(|variant| {
            let Variant { attrs, fields, .. } = variant;

            let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);

            ProxyVariant {
                input: (*variant).clone(),
                reserved_attrs: surplus,
                conf_fields: parse_conf_fields(fields, &matched, ATTR_PROXY_ENUM, ATTR_INCLUDES),
                string_enum,
            }
        })
        .collect()
}

impl ToTokens for ProxyVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { input, reserved_attrs, conf_fields, string_enum, .. } = self;
        let Variant { ident, .. } = input;
        let ConfFields { fields, .. } = conf_fields;

        if *string_enum {
            append_to_tokens(tokens, quote_spanned! { input.span() =>
                #( #reserved_attrs )* #ident
            });
            return;
        }
        append_to_tokens(tokens, quote_spanned! { input.span() =>
            #( #reserved_attrs )*
            #ident { #( #fields ),* }
        });
    }
}