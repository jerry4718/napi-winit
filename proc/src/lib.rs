#![allow(unused_imports, unused_variables, dead_code)]

mod mapping_bitflags;
mod utils;
mod proxy_enum;
mod proxy_flags;

use proc_macro::TokenStream;

#[proc_macro]
pub fn mapping_bitflags(input: TokenStream) -> TokenStream {
    mapping_bitflags::mapping_bitflags(input)
}

#[proc_macro_attribute]
pub fn proxy_enum(attrs: TokenStream, input: TokenStream) -> TokenStream {
    proxy_enum::proxy_enum(attrs, input)
}

#[proc_macro_attribute]
pub fn proxy_flags(attrs: TokenStream, input: TokenStream) -> TokenStream {
    proxy_flags::proxy_flags(attrs, input)
}