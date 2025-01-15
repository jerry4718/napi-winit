use quote::ToTokens;
use syn::{parse_str, LitStr};

#[inline]
pub fn to_lit_str(tokens: Box<dyn ToTokens>) -> LitStr {
    let string = tokens.into_token_stream().to_string();
    let inner = vec![String::from('"'), string, String::from('"')];
    parse_str::<LitStr>(&inner.join("")).unwrap()
}
