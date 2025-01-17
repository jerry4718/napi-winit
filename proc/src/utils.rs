use quote::ToTokens;
use syn::{parse_str, Attribute, LitStr};

#[inline]
pub fn to_lit_str(tokens: Box<dyn ToTokens>) -> LitStr {
    let string = tokens.into_token_stream().to_string();
    let inner = vec![String::from('"'), string, String::from('"')];
    parse_str::<LitStr>(&inner.join("")).unwrap()
}

#[inline]
pub fn find_attr_by_name(attrs: &Vec<Attribute>, name: &str) -> Option<Attribute> {
    for attr in attrs {
        if matches!(attr.path().get_ident(), Some(ident) if ident == name) {
            return Some(attr.clone());
        }
    }
    None
}