mod mapping_enum;
mod mapping_bitflags;
mod utils;

use proc_macro::TokenStream;

#[proc_macro]
pub fn mapping_enum(input: TokenStream) -> TokenStream {
    mapping_enum::mapping_enum(input)
}

#[proc_macro]
pub fn mapping_bitflags(input: TokenStream) -> TokenStream {
    mapping_bitflags::mapping_bitflags(input)
}