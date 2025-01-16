mod mapping_struct;
mod mapping_enum;
mod mapping_bitflags;
mod utils;
mod simple_enum;
mod simple_struct;

use proc_macro::TokenStream;

#[proc_macro]
pub fn mapping_enum(input: TokenStream) -> TokenStream {
    mapping_enum::mapping_enum(input)
}

#[proc_macro]
pub fn simple_enum(input: TokenStream) -> TokenStream {
    simple_enum::simple_enum(input)
}

#[proc_macro]
pub fn mapping_struct(input: TokenStream) -> TokenStream {
    mapping_struct::mapping_struct(input)
}

#[proc_macro]
pub fn simple_struct(input: TokenStream) -> TokenStream {
    simple_struct::simple_struct(input)
}

#[proc_macro]
pub fn mapping_bitflags(input: TokenStream) -> TokenStream {
    mapping_bitflags::mapping_bitflags(input)
}