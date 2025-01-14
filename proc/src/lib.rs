mod mapping_struct;
mod mapping_enum;

use proc_macro::TokenStream;


#[proc_macro]
pub fn mapping_enum(input: TokenStream) -> TokenStream {
    mapping_enum::mapping_enum(input)
}

#[proc_macro]
pub fn mapping_struct(input: TokenStream) -> TokenStream {
    mapping_struct::mapping_struct(input)
}