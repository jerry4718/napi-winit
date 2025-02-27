use crate::utils::{append_to_tokens, get_meta_value_as_block, get_meta_value_as_lit_str, get_meta_value_as_type, get_metas_by_attr_name, get_type_ty_or, parse_as, parse_metas, separate_attr_by_name, to_tokens};
use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
    quote_spanned,
    IdentFragment,
    ToTokens
};
use syn::{
    parse_macro_input,
    spanned::Spanned,
    Attribute,
    ExprBlock,
    Field,
    Fields,
    Ident,
    ItemEnum,
    Meta,
    Path,
    PathSegment,
    Type,
    TypePath,
    Variant
};

pub(crate) fn proxy_enum(attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let metas = parse_metas(attrs);

    let item_enum = parse_macro_input!(input as ItemEnum);

    let out_quotes = temp_enum(&metas, &item_enum);

    proc_macro::TokenStream::from(quote! { #out_quotes })
}

struct TempEnum {
    pub input: ItemEnum,
    pub reserved_attrs: Vec<Attribute>,
    pub temp_variants: Vec<TempVariant>,
    pub origin_enum: Type,
    pub skip_forward: bool,
    pub skip_backward: bool,
}

enum FieldsLike {
    Named,
    Unnamed,
    Unit,
}

impl From<&Fields> for FieldsLike {
    fn from(value: &Fields) -> Self {
        match value {
            Fields::Named(_) => Self::Named,
            Fields::Unnamed(_) => Self::Unnamed,
            Fields::Unit => Self::Unit,
        }
    }
}

struct TempVariant {
    pub input: Variant,
    pub like: FieldsLike,
    pub reserved_attrs: Vec<Attribute>,
    pub temp_fields: Vec<TempField>,
}

struct TempField {
    pub input: Field,
    pub reserved_attrs: Vec<Attribute>,
    pub ident: Ident,
    pub direct_type: bool,
    pub use_clone: bool,
    pub from_origin: Option<ExprBlock>,
    pub into_origin: Option<ExprBlock>,
}

enum With { Origin, Proxy }
enum Kind { Compose, Dispose }

fn map_temp_fields<F>(variant: &TempVariant, pose: Kind, with: With, mapper: F) -> TokenStream
where
    F: Fn(&TempField) -> TokenStream,
{
    let TempVariant { temp_fields, .. } = variant;
    let field_idents: Vec<TokenStream> = temp_fields.iter().map(quote_ident).collect();
    let field_tokens: Vec<TokenStream> = temp_fields.iter().map(mapper).collect();

    match (with, pose, &variant.like) {
        (With::Origin, Kind::Dispose, FieldsLike::Named) => quote!({ #( #field_idents ),* }),
        (With::Origin, Kind::Compose, FieldsLike::Named) => quote!({ #( #field_idents: #field_tokens ),* }),
        (With::Origin, _, FieldsLike::Unnamed) => quote!(( #( #field_tokens ),* )),
        (With::Origin, _, FieldsLike::Unit) => quote!(),
        (With::Proxy, Kind::Dispose, _) => quote!({ #( #field_idents ),* }),
        (With::Proxy, Kind::Compose, _) => quote!({ #( #field_idents: #field_tokens ),* }),
    }
}

fn quote_ident(TempField { ident, .. }: &TempField) -> TokenStream {
    quote! { #ident }
}

fn conv_from_branch((variant, origin_type): (&TempVariant, &Type)) -> TokenStream {
    let TempVariant { input: Variant { ident, .. }, .. } = variant;
    let dispose = map_temp_fields(
        variant, Kind::Dispose, With::Origin,
        |TempField { ident, .. }| quote! { #ident },
    );
    let compose = map_temp_fields(
        variant, Kind::Compose, With::Proxy,
        |TempField { ident, from_origin, .. }| {
            match from_origin {
                Some(from_block) => quote! { #from_block },
                None => quote! { #ident.into() },
            }
        },
    );
    quote! { #origin_type::#ident #dispose => Self::#ident #compose }
}

fn conv_into_branch((variant, origin_type): (&TempVariant, &Type)) -> TokenStream {
    let TempVariant { input: Variant { ident, .. }, .. } = variant;
    let dispose = map_temp_fields(
        variant, Kind::Dispose, With::Proxy,
        |TempField { ident, .. }| quote! { #ident },
    );
    let compose = map_temp_fields(
        variant, Kind::Compose, With::Origin,
        |TempField { ident, into_origin, .. }| {
            match into_origin {
                Some(into_block) => quote! { #into_block },
                None => quote! { #ident.into() },
            }
        },
    );
    quote! { Self::#ident #dispose => #origin_type::#ident #compose }
}

impl ToTokens for TempEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { input, reserved_attrs, temp_variants, origin_enum: origin_type, skip_forward, skip_backward } = self;

        let ItemEnum { ident, vis, .. } = input;

        let mut napi_metas = Vec::new();
        if *skip_forward { napi_metas.push(quote! {object_to_js = false}) }
        if *skip_backward { napi_metas.push(quote! {object_from_js = false}) }

        append_to_tokens(tokens, quote_spanned! { input.span() =>
            #[napi( #( #napi_metas ),* )]
            #( #reserved_attrs )*
            #vis enum #ident {
                #( #temp_variants ),*
            }
        });

        if !skip_forward {
            let conv_from: Vec<_> = temp_variants
                .iter()
                .zip(vec![origin_type; temp_variants.len()])
                .map(conv_from_branch)
                .collect();

            append_to_tokens(tokens, quote! {
                impl From<#origin_type> for #ident {
                    fn from(value: #origin_type) -> Self {
                        match value {
                            #( #conv_from ),*
                        }
                    }
                }
            });
        }

        if !skip_backward {
            let conv_into: Vec<_> = temp_variants
                .iter()
                .zip(vec![origin_type; temp_variants.len()])
                .map(conv_into_branch)
                .collect();

            append_to_tokens(tokens, quote! {
                impl Into<#origin_type> for #ident {
                    fn into(self) -> #origin_type {
                        match self {
                            #( #conv_into ),*
                        }
                    }
                }
            });
        }
    }
}

impl ToTokens for TempVariant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { input, like, reserved_attrs, temp_fields } = self;
        let Variant { ident, fields, .. } = input;

        append_to_tokens(tokens, quote_spanned! { input.span() =>
            #( #reserved_attrs )*
            #ident { #( #temp_fields ),* }
        });
    }
}

impl ToTokens for TempField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { input, reserved_attrs, ident, .. } = self;
        let Field { vis, ty, .. } = input;

        append_to_tokens(tokens, quote_spanned! { input.span() =>
            #( #reserved_attrs )*
            #vis #ident: #ty
        });
    }
}

const ATTR_PROXY_ENUM: &str = "proxy_enum";

const ATTR_INCLUDES: &[&str] = &[
    ATTR_PROXY_ENUM,
];

const META_ORIGIN_ENUM: &str = "origin_enum";
const META_SKIP_FORWARD: &str = "skip_forward";
const META_SKIP_BACKWARD: &str = "skip_backward";
const META_FIELD_NAME: &str = "field_name";
const META_FROM_ORIGIN: &str = "from_origin";
const META_INTO_ORIGIN: &str = "into_origin";
const META_DIRECT_TYPE: &str = "direct_type";
const META_USE_CLONE: &str = "use_clone";

const META_NAMED: &str = "Named";
const META_UNNAMED: &str = "Unnamed";

macro_rules! map_meta_to_local {
    ($from:expr => { $($name:expr => $local:ident),* $(,)? }) => {
        let metas = ($from);
        $(let $local = $crate::utils::get_meta_by_name(metas, $name);)*
    };
}

fn temp_enum(metas: &Vec<Meta>, item_enum: &ItemEnum) -> TempEnum {
    let ItemEnum { attrs, ident, variants, generics, .. } = item_enum;

    let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);
    if matches!(matched.len(), n if n > 0) {
        panic!("so many proxy_enum");
    }

    map_meta_to_local!(&metas => {
        META_ORIGIN_ENUM => origin_enum,
        META_SKIP_FORWARD => skip_forward,
        META_SKIP_BACKWARD => skip_backward,
    });

    let origin_enum = get_type_ty_or(&origin_enum, &format_ident!("Origin{}", ident));

    TempEnum {
        input: item_enum.clone(),
        reserved_attrs: surplus,
        temp_variants: temp_variants(variants.iter().collect()),
        origin_enum,
        skip_forward: skip_forward.is_some(),
        skip_backward: skip_backward.is_some(),
    }
}

fn temp_variants(variants: Vec<&Variant>) -> Vec<TempVariant> {
    variants.iter()
        .map(|variant| {
            let Variant { attrs, fields, ident, .. } = variant;

            let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);

            map_meta_to_local!(&get_metas_by_attr_name(&matched, ATTR_PROXY_ENUM) => {
                META_NAMED => meta_named,
                META_UNNAMED => meta_unnamed,
            });

            let field_like = match (meta_named, meta_unnamed) {
                (None, None) => FieldsLike::from(fields),
                (Some(_), Some(_)) => panic!("unexpected both Named and Unnamed"),
                (Some(_), _) => FieldsLike::Named,
                (_, Some(_)) => FieldsLike::Unnamed,
            };

            TempVariant {
                input: (*variant).clone(),
                like: field_like,
                reserved_attrs: surplus,
                temp_fields: temp_fields(fields.iter().collect()),
            }
        })
        .collect()
}

fn temp_fields(fields: Vec<&Field>) -> Vec<TempField> {
    fields.iter()
        .zip(0..fields.len())
        .map(|(field, idx)| {
            let Field { attrs, ident, ty, .. } = field;

            let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);

            map_meta_to_local!(&get_metas_by_attr_name(&matched, ATTR_PROXY_ENUM) => {
                META_FIELD_NAME => field_name,
                META_DIRECT_TYPE => direct_type,
                META_USE_CLONE => use_clone,
                META_FROM_ORIGIN => from_origin,
                META_INTO_ORIGIN => into_origin,
            });

            let ident = match (ident, field_name) {
                (None, None) => format_ident!("field_{}", idx),
                (Some(ident), None) => ident.clone(),
                (_, Some(meta)) => {
                    let Some(lit) = get_meta_value_as_lit_str(&meta)
                    else { panic!("must assign a string literal") };
                    format_ident!("{}", lit.value())
                }
            };

            let from_origin = from_origin.map(|meta| get_meta_value_as_block(&meta)).flatten();
            let into_origin = into_origin.map(|meta| get_meta_value_as_block(&meta)).flatten();

            TempField {
                input: (*field).clone(),
                reserved_attrs: surplus,
                ident,
                direct_type: direct_type.is_some(),
                use_clone: use_clone.is_some(),
                from_origin,
                into_origin,
            }
        })
        .collect()
}