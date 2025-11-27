#![allow(unused_imports, unused_variables, dead_code)]

use macros::for_proc;

mod utils;
mod conf_fields;
mod conf_convert;
mod proxy_enum;
mod proxy_struct;
mod proxy_wrap;
mod proxy_impl;
mod proxy_flags;
mod conf_usage;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn proxy_enum(attrs: TokenStream, input: TokenStream) -> TokenStream {
    proxy_enum::proxy_enum(attrs, input)
}

#[proc_macro_attribute]
pub fn proxy_struct(attrs: TokenStream, input: TokenStream) -> TokenStream {
    proxy_struct::proxy_struct(attrs, input)
}

#[proc_macro_attribute]
pub fn proxy_wrap(attrs: TokenStream, input: TokenStream) -> TokenStream {
    proxy_wrap::proxy_wrap(attrs, input)
}

#[proc_macro_attribute]
pub fn proxy_impl(attrs: TokenStream, input: TokenStream) -> TokenStream {
    proxy_impl::proxy_impl(attrs, input)
}

#[proc_macro_attribute]
pub fn proxy_flags(attrs: TokenStream, input: TokenStream) -> TokenStream {
    proxy_flags::proxy_flags(attrs, input)
}