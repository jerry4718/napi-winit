use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Parser},
    Attribute, Ident, Meta, MetaNameValue, Token, Type,
};

mod meta_getter;
pub use meta_getter::*;

pub(crate) fn parse_as<As: Parse>(to_tokens: &dyn ToTokens) -> As {
    try_parse2(to_tokens.to_token_stream(), As::parse)
        .expect(format!("failed to parse as {}", std::any::type_name::<As>()).as_str())
}

fn try_parse<P: Parser>(tokens: proc_macro::TokenStream, p: P) -> syn::Result<P::Output> {
    Parser::parse(p, tokens)
}

fn try_parse2<P: Parser>(tokens: TokenStream, p: P) -> syn::Result<P::Output> {
    Parser::parse2(p, tokens)
}

pub(crate) fn parse_metas(input: proc_macro::TokenStream) -> Vec<Meta> {
    try_parse(input, parse_to_metas).unwrap_or_else(|_| vec![])
}

fn parse_to_metas(input: ParseStream) -> syn::Result<Vec<Meta>> {
    let mut metas = Vec::<Meta>::new();
    while let Ok(meta) = input.parse::<Meta>() {
        input.parse::<Option<Token![,]>>()?;
        metas.push(meta);
    }
    Ok(metas)
}

pub(crate) fn get_metas_by_attr_name(attrs: &Vec<Attribute>, name: &str) -> Vec<Meta> {
    attrs.iter()
        .filter(|attr| matches!(attr.path().get_ident(), Some(ident) if ident == name))
        .map(|attr| attr.parse_args_with(parse_to_metas).unwrap_or(vec![]))
        .flatten()
        .collect()
}

pub(crate) fn get_meta_by_name(metas: &Vec<Meta>, name: &str) -> Option<Meta> {
    metas.iter()
        .find(|meta| matches!(meta.path().get_ident(), Some(ident) if ident == name))
        .cloned()
}

pub(crate) fn get_meta_value_as<As: Parse>(meta: &Meta) -> Option<As> {
    match meta {
        Meta::NameValue(MetaNameValue { ref value, .. }) => Some(parse_as::<As>(value)),
        _ => None,
    }
}

pub(crate) fn get_type_ty_or<Input: ToTokens>(meta: &Option<Meta>, input: &Input) -> Type {
    meta.as_ref()
        .map(get_meta_value_as::<Type>)
        .flatten()
        .or_else(|| Some(parse_as::<Type>(input)))
        .expect("cannot parse meta value as type")
}

pub(crate) fn get_ident_optional(meta: &Option<Meta>) -> Option<Ident> {
    let Some(meta) = meta else { return None };
    let Some(ident) = get_meta_value_as(&meta)
    else { panic!("cannot parse meta value as ident"); };
    Some(ident)
}

pub(crate) fn separate_attr_by_name(attrs: &Vec<Attribute>, names: &[&str]) -> (Vec<Attribute>, Vec<Attribute>) {
    let mut matched = Vec::new();
    let mut reserved = Vec::new();

    for attr in attrs {
        match attr.path().get_ident() {
            Some(ident) if names.contains(&ident.to_string().as_str()) => matched.push(attr.clone()),
            _ => reserved.push(attr.clone())
        }
    }
    (matched, reserved)
}

pub(crate) fn append_to_tokens<T: ToTokens>(dst: &mut TokenStream, from: T) {
    from.to_tokens(dst);
}