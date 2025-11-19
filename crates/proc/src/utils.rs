use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{parse, Attribute, ExprBlock, ExprClosure, ExprTuple, Ident, LitStr, Meta, MetaList, MetaNameValue, Token, Type};

pub(crate) struct Metas {
    metas: Vec<Meta>,
}

impl Parse for Metas {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut metas = Vec::<Meta>::new();

        while let Ok(meta) = input.parse::<Meta>() {
            input.parse::<Option<Token![,]>>()?;
            metas.push(meta);
        }
        Ok(Metas { metas })
    }
}

#[inline]
pub(crate) fn parse_metas(input: proc_macro::TokenStream) -> Vec<Meta> {
    let Ok(Metas { metas }) = parse::<Metas>(input)
    else { panic!("cannot be parsed as list of meta") };
    metas
}

#[inline]
pub(crate) fn parse_as<As: Parse + Spanned>(to_tokens: &dyn ToTokens) -> As {
    parse::<As>(proc_macro::TokenStream::from(to_tokens.into_token_stream())).unwrap()
}

#[inline]
pub(crate) fn get_attr_by_name(attrs: &Vec<Attribute>, name: &str) -> Option<Attribute> {
    for attr in attrs {
        if matches!(attr.path().get_ident(), Some(ident) if ident == name) {
            return Some(attr.clone());
        }
    }
    None
}

#[inline]
pub(crate) fn get_attr_metas(attr: &Option<Attribute>) -> Vec<Meta> {
    let Some(attr) = attr else { return Vec::new() };
    let Meta::List(MetaList { tokens, .. }) = &attr.meta else { return Vec::new() };
    parse_metas(proc_macro::TokenStream::from(tokens.clone()))
}

#[inline]
pub(crate) fn get_metas_by_attr_name(attrs: &Vec<Attribute>, name: &str) -> Vec<Meta> {
    get_attr_metas(&get_attr_by_name(attrs, name))
}

#[inline]
pub(crate) fn get_meta_by_name(metas: &Vec<Meta>, name: &str) -> Option<Meta> {
    for meta in metas {
        if matches!(meta.path().get_ident(), Some(ident) if ident == name) {
            return Some(meta.clone());
        }
    }
    None
}

pub(crate) trait MetaGetter<T> {
    type Output;
    fn get(&self, metas: &Vec<Meta>) -> Self::Output;
}

macro_rules! seat_tt {
    ($name: ident => $($tt: tt)*) => { $($tt)* };
}

macro_rules! impl_meta_getter {
    ($($name:ident),*) => {
        impl MetaGetter<Self> for ($(seat_tt!( $name => &str )),*,) {
            type Output = ($(seat_tt!( $name => Option<Meta> )),*,);

            fn get(&self, metas: &Vec<Meta>) -> Self::Output {
                let ($($name),*,) = self;
                ($(get_meta_by_name(metas, $name)),*,)
            }
        }
    };
}

impl_meta_getter!(name0);
impl_meta_getter!(name0, name1);
impl_meta_getter!(name0, name1, name2);
impl_meta_getter!(name0, name1, name2, name3);
impl_meta_getter!(name0, name1, name2, name3, name4);
impl_meta_getter!(name0, name1, name2, name3, name4, name5);
impl_meta_getter!(name0, name1, name2, name3, name4, name5, name6);
impl_meta_getter!(name0, name1, name2, name3, name4, name5, name6, name7);
impl_meta_getter!(name0, name1, name2, name3, name4, name5, name6, name7, name8);
impl_meta_getter!(name0, name1, name2, name3, name4, name5, name6, name7, name8, name9);

#[inline]
pub(crate) fn get_meta_by_names<T: MetaGetter<T>>(metas: &Vec<Meta>, names: T) -> <T as MetaGetter<T>>::Output {
    names.get(metas)
}

#[inline]
pub(crate) fn get_meta_value_as_type(meta: &Meta) -> Option<Type> {
    let Meta::NameValue(MetaNameValue { ref value, .. }) = meta
    else { return None };

    Some(parse_as::<Type>(value))
}

#[inline]
pub(crate) fn get_meta_value_as_ident(meta: &Meta) -> Option<Ident> {
    let Meta::NameValue(MetaNameValue { ref value, .. }) = meta
    else { return None };

    Some(parse_as::<Ident>(value))
}

#[inline]
pub(crate) fn get_meta_value_as_lit_str(meta: &Meta) -> Option<LitStr> {
    let Meta::NameValue(MetaNameValue { ref value, .. }) = meta
    else { return None };

    Some(parse_as::<LitStr>(value))
}
#[inline]
pub(crate) fn get_meta_value_as_expr_tuple(meta: &Meta) -> Option<ExprTuple> {
    let Meta::NameValue(MetaNameValue { ref value, .. }) = meta
    else { return None };

    Some(parse_as::<ExprTuple>(value))
}

#[inline]
pub(crate) fn get_meta_value_as_closure(meta: &Meta) -> Option<ExprClosure> {
    let Meta::NameValue(MetaNameValue { ref value, .. }) = meta
    else { return None };

    Some(parse_as::<ExprClosure>(value))
}

#[inline]
pub(crate) fn get_meta_value_as_block(meta: &Meta) -> Option<ExprBlock> {
    let Meta::NameValue(MetaNameValue { ref value, .. }) = meta
    else { return None };

    Some(parse_as::<ExprBlock>(value))
}

#[inline]
pub(crate) fn get_type_ty_or<Input: ToTokens>(meta: &Option<Meta>, input: &Input) -> Type {
    match meta {
        None => parse_as::<Type>(input),
        Some(meta) => {
            let Some(ty) = get_meta_value_as_type(&meta)
            else { panic!("cannot parse meta value as type"); };
            ty
        }
    }
}

#[inline]
pub(crate) fn get_ident_optional(meta: &Option<Meta>) -> Option<Ident> {
    let Some(meta) = meta else { return None };
    let Some(ident) = get_meta_value_as_ident(&meta)
    else { panic!("cannot parse meta value as ident"); };
    Some(ident)
}

#[inline]
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

#[inline]
pub(crate) fn append_to_tokens<T: ToTokens>(dst: &mut TokenStream, from: T) {
    from.to_tokens(dst);
}