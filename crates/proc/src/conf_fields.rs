use crate::{
    conf_usage::{get_meta_value_as_conf_usage, ConfUsage},
    utils::{append_to_tokens, get_meta_value_as_ident, get_metas_by_attr_name, separate_attr_by_name}
};
use macros::{define_const_str, map_meta_to_local};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::spanned::Spanned;
use syn::{Attribute, Field, Fields, Ident};

pub(crate) struct ConfFields {
    pub like: FieldsLike,
    pub fields: Vec<ConfField>,
}

pub(crate) struct ConfField {
    pub input: Field,
    pub reserved_attrs: Vec<Attribute>,
    pub field_ident: Ident,
    pub from_origin: Option<ConfUsage>,
    pub into_origin: Option<ConfUsage>,
}

const META_FIELD_NAME: &str = "field_name";
const META_FROM_ORIGIN: &str = "from_origin";
const META_INTO_ORIGIN: &str = "into_origin";

pub(crate) fn parse_conf_fields(fields: &Fields, parent_attrs: &Vec<Attribute>, attr_proxy_name: &str, attr_includes: &[&str]) -> ConfFields {
    ConfFields {
        like: parse_fields_like(fields, parent_attrs, attr_proxy_name),
        fields: fields.iter()
            .zip(0..fields.len())
            .map(|(field, idx)| {
                let Field { attrs, ident, .. } = field;

                let (matched, surplus) = separate_attr_by_name(attrs, attr_includes);

                map_meta_to_local!(&get_metas_by_attr_name(&matched, attr_proxy_name) => {
                    META_FIELD_NAME => field_name,
                    META_FROM_ORIGIN => from_origin,
                    META_INTO_ORIGIN => into_origin,
                });

                let field_ident = field_name
                    .map(|meta| get_meta_value_as_ident(&meta))
                    .flatten()
                    .or_else(|| ident.clone())
                    .unwrap_or_else(|| format_ident!("field_{}", idx));

                let from_origin = from_origin.map(|meta| get_meta_value_as_conf_usage(&meta)).flatten();
                let into_origin = into_origin.map(|meta| get_meta_value_as_conf_usage(&meta)).flatten();

                ConfField {
                    input: field.clone(),
                    reserved_attrs: surplus,
                    field_ident,
                    from_origin,
                    into_origin,
                }
            })
            .collect(),
    }
}

impl ToTokens for ConfField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { input, reserved_attrs, field_ident: ident, .. } = self;
        let Field { vis, ty, .. } = input;

        append_to_tokens(tokens, quote_spanned! { input.span() =>
            #( #reserved_attrs )*
            #vis #ident: #ty
        });
    }
}

pub(crate) enum FieldsLike {
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

pub(crate) enum With { Origin, Proxy }
pub(crate) enum Kind { Compose, Dispose }

pub(crate) fn quote_conf_fields<F>(ConfFields { fields, like }: &ConfFields, pose: Kind, with: With, mapper: F) -> TokenStream
where
    F: Fn(&ConfField) -> TokenStream,
{
    let field_idents: Vec<TokenStream> = fields.iter().map(quote_ident).collect();
    let field_tokens: Vec<TokenStream> = fields.iter().map(mapper).collect();

    match (with, pose, like) {
        (With::Origin, Kind::Dispose, FieldsLike::Named) => quote!({ #( #field_idents, )* .. }),
        (With::Origin, Kind::Compose, FieldsLike::Named) => quote!({ #( #field_idents: #field_tokens ),* }),
        (With::Origin, _, FieldsLike::Unnamed) => quote!(( #( #field_tokens ),* )),
        (With::Origin, _, FieldsLike::Unit) => quote!(),
        (With::Proxy, Kind::Dispose, _) => quote!({ #( #field_idents, )* .. }),
        (With::Proxy, Kind::Compose, _) => quote!({ #( #field_idents: #field_tokens ),* }),
    }
}

fn quote_ident(ConfField { field_ident, .. }: &ConfField) -> TokenStream {
    quote! { #field_ident }
}

define_const_str!(
    META_NAMED = Named,
    META_UNNAMED = Unnamed,
);

pub(crate) fn parse_fields_like(fields: &Fields, attrs: &Vec<Attribute>, attr_proxy_name: &str) -> FieldsLike {
    map_meta_to_local!(&get_metas_by_attr_name(&attrs, attr_proxy_name) => {
        META_NAMED => meta_named,
        META_UNNAMED => meta_unnamed,
    });

    match (meta_named, meta_unnamed) {
        (None, None) => FieldsLike::from(fields),
        (Some(_), Some(_)) => panic!("unexpected both Named and Unnamed"),
        (Some(_), _) => FieldsLike::Named,
        (_, Some(_)) => FieldsLike::Unnamed,
    }
}