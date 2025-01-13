mod enum_dsl;
mod struct_dsl;
mod enum_to_mod;

use proc_macro::TokenStream;

#[proc_macro]
pub fn enum_dsl(input: TokenStream) -> TokenStream {
    enum_dsl::enum_dsl(input)
}

#[proc_macro]
pub fn enum_to_mod(input: TokenStream) -> TokenStream {
    enum_to_mod::enum_to_mod(input)
}

#[proc_macro]
pub fn struct_dsl(input: TokenStream) -> TokenStream {
    struct_dsl::struct_dsl(input)
}