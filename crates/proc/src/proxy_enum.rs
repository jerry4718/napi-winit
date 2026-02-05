use std::{
    fmt::format,
    mem::transmute,
};
use crate::{
    utils::{append_to_tokens, get_meta_value_as, get_type_ty_or, parse_metas, separate_attr_by_name},
    conf_convert::{parse_conf_convert, ConfConvert, NormalConfConvert},
    conf_fields::{parse_conf_fields, quote_conf_fields, ConfField, ConfFields, Kind, With},
    conf_usage::quote_option_conf_usage,
    utils::parse_as,
};
use macros::{define_const_str, map_meta_to_local};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, Attribute, BinOp, Expr, ExprBinary, Field, Fields, FieldsNamed, FieldsUnnamed, ItemEnum, LitInt, Meta, Token, Type, Variant, Error, Path};
use crate::utils::get_metas_by_attr_name;

pub(crate) fn proxy_enum(attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let metas = parse_metas(attrs);
    let item_enum = parse_macro_input!(input as ItemEnum);

    match parse_proxy_enum(&metas, &item_enum) {
        Ok(proxy_enum) => proc_macro::TokenStream::from(quote! { #proxy_enum }),
        Err(err) => err.to_compile_error().into(),
    }
}

define_const_str!(ATTR_PROXY_ENUM = proxy_enum);
const ATTR_INCLUDES: &[&str] = &[ATTR_PROXY_ENUM];

define_const_str!(
    META_ORIGIN_ENUM = origin_type,
    META_STRING_ENUM = string_enum,
    META_NON_EXHAUSTIVE = non_exhaustive,
    META_CODE_NAME = code_name,
);

struct ProxyEnum {
    pub input: ItemEnum,
    pub reserved_attrs: Vec<Attribute>,
    pub proxy_variants: Vec<ProxyVariant>,
    pub origin_type: Type,
    pub string_enum: bool,
    pub non_exhaustive: bool,
    pub conf_convert: ConfConvert,
    pub conf_code: Option<ConfCode>,
}

#[derive(Clone)]
struct ConfCode {
    code_name: Ident,
    code_type: Type,
}

enum ProxyVariants {
    Structured(Vec<ProxyVariant>),
    Constants(Vec<ProxyVariant>),
}

fn get_repr_type(attrs: &Vec<Attribute>) -> Type {
    get_metas_by_attr_name(attrs, "repr").first()
        .map(Meta::path)
        .and_then(Path::get_ident)
        .map(|ident| parse_as::<Type>(ident))
        .unwrap_or_else(|| parse_as::<Type>(&quote!(u8)))
}

fn parse_proxy_enum(metas: &Vec<Meta>, item_enum: &ItemEnum) -> Result<ProxyEnum, Error> {
    let ItemEnum { attrs, ident, variants, .. } = item_enum;

    let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);
    if matches!(matched.len(), n if n > 0) {
        return Err(Error::new(item_enum.span(), "so many proxy_enum attributes"));
    }

    map_meta_to_local!(&metas => {
        META_ORIGIN_ENUM => origin_type,
        META_STRING_ENUM => string_enum,
        META_NON_EXHAUSTIVE => non_exhaustive,
        META_CODE_NAME => code_name,
    });

    let code_name = code_name.as_ref()
        .and_then(get_meta_value_as::<Ident>)
        .map(|code_name| ConfCode { code_name, code_type: get_repr_type(&surplus) })
        .or_else(|| {
            let has_discriminant = variants.iter()
                .any(|Variant { discriminant, .. }| discriminant.is_some());
            if !has_discriminant { return None; }

            Some(ConfCode {
                code_name: format_ident!("discriminant"),
                code_type: get_repr_type(&surplus),
            })
        });

    let proxy_variants = parse_proxy_variants(string_enum.is_some(), &code_name, variants.iter().collect())?;

    Ok(ProxyEnum {
        input: item_enum.clone(),
        reserved_attrs: surplus,
        proxy_variants,
        origin_type: get_type_ty_or(&origin_type, &format_ident!("Origin{}", ident)),
        string_enum: string_enum.is_some(),
        non_exhaustive: non_exhaustive.is_some(),
        conf_convert: parse_conf_convert(metas),
        conf_code: code_name,
    })
}

impl ToTokens for ProxyEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            input, reserved_attrs, proxy_variants,
            origin_type, string_enum, non_exhaustive, conf_convert, conf_code
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
            let conv_from: Vec<_> = proxy_variants
                .iter()
                .map(|variant| {
                    if conf_code.is_some() {
                        return conv_const_from(variant, origin_type);
                    }
                    if !*string_enum {
                        return conv_structure_from(variant, origin_type);
                    }
                    conv_unit_from(variant, origin_type)
                })
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
            let conv =
                if !*string_enum { conv_structure_into } else { conv_unit_into };
            let conv_into: Vec<_> = proxy_variants
                .iter()
                .map(|variant| {
                    if conf_code.is_some() {
                        return conv_const_into(variant, origin_type);
                    }
                    if !*string_enum {
                        return conv_structure_into(variant, origin_type);
                    }
                    conv_unit_into(variant, origin_type)
                })
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

fn conv_const_from(variant: &ProxyVariant, origin_type: &Type) -> TokenStream {
    let ProxyVariant {
        input: Variant { ident, fields, .. },
        conf_code: Some(ConfCode { code_name, code_type }),
        ..
    } = variant
    else {
        return quote! { compile_error!("Unexpected logic error in conv_const_from") };
    };

    let fields_pat = match fields {
        Fields::Named(_) => quote! { { .. } },
        Fields::Unnamed(_) => quote! { ( .. ) },
        Fields::Unit => quote! {},
    };

    quote! { #origin_type::#ident #fields_pat => Self::#ident { #code_name: unsafe { *<*const _>::from(&value).cast::<#code_type>() } } }
}

fn conv_const_into(variant: &ProxyVariant, origin_type: &Type) -> TokenStream {
    let ProxyVariant {
        input: Variant { ident, .. },
        conf_code: Some(ConfCode { code_name, .. }),
        ..
    } = variant
    else {
        return quote! { compile_error!("Unexpected logic error in conv_const_into") };
    };

    quote! { Self::#ident { #code_name: _, .. } => #origin_type::#ident }
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
    pub conf_code: Option<ConfCode>,
}

fn parse_proxy_variants(string_enum: bool, code_info: &Option<ConfCode>, variants: Vec<&Variant>) -> Result<Vec<ProxyVariant>, Error> {
    let proxy_variants = variants.iter()
        .map(|variant| {
            let Variant { attrs, fields, .. } = variant;

            let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);

            ProxyVariant {
                input: (*variant).clone(),
                reserved_attrs: surplus,
                conf_fields: parse_conf_fields(fields, &matched, ATTR_PROXY_ENUM, ATTR_INCLUDES),
                string_enum,
                conf_code: code_info.clone(),
            }
        })
        .collect::<Vec<_>>();

    Ok(proxy_variants)
}

impl ToTokens for ProxyVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { input, reserved_attrs, conf_fields, string_enum, conf_code, .. } = self;
        let Variant { ident, discriminant, .. } = input;
        let ConfFields { fields, .. } = conf_fields;

        if let Some(ConfCode { code_name, code_type, .. }) = conf_code {
            let mut napi_metas = Vec::new();

            if let Some((_, Expr::Lit(lit))) = discriminant {
                let expr = Literal::string(&*lit.into_token_stream().to_string());
                napi_metas.push(quote! { ts_type = #expr });
            }

            append_to_tokens(tokens, quote_spanned! { input.span() =>
                #( #reserved_attrs )* #ident {
                    #[napi( #( #napi_metas ),* )]
                    #code_name: #code_type
                }
            });

            return;
        }

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