#![allow(unused_imports, unused_variables, dead_code)]

mod utils;
mod proxy_enum;
mod proxy_flags;
mod proxy_wrap;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn proxy_enum(attrs: TokenStream, input: TokenStream) -> TokenStream {
    proxy_enum::proxy_enum(attrs, input)
}

#[proc_macro_attribute]
pub fn proxy_wrap(attrs: TokenStream, input: TokenStream) -> TokenStream {
    proxy_wrap::proxy_wrap(attrs, input)
}

#[proc_macro_attribute]
pub fn proxy_flags(attrs: TokenStream, input: TokenStream) -> TokenStream {
    proxy_flags::proxy_flags(attrs, input)
}