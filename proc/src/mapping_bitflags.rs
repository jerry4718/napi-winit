use proc_macro::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{parse_macro_input, Ident, Token, braced};
use syn::parse::{Parse, ParseStream};
use crate::utils::to_lit_str;

struct FlagsInputSpec {
    name: Ident,
    flags: Vec<Ident>
}

impl Parse for FlagsInputSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _struct = input.parse::<Token![struct]>()?;
        let name = input.parse::<Ident>()?;

        let content;
        let _brace_token = braced!(content in input);

        let mut flags = Vec::<Ident>::new();

        while let Some(flag) = content.parse::<Option<Ident>>()? {
            content.parse::<Option<Token![;]>>()?;
            flags.push(flag);
        }
        Ok(Self { name, flags })
    }
}

pub fn mapping_bitflags(input: TokenStream) -> TokenStream {
    let FlagsInputSpec { name, flags } = parse_macro_input!(input as FlagsInputSpec);

    let export_ident = name.clone();
    let export_ident_lit = to_lit_str(Box::new(export_ident.clone()));

    let origin_ident = format_ident!("Origin{}", name);

    let origin_ident = quote_spanned! { name.span() => #origin_ident };

    let lower_flags: Vec<_> = flags.iter().map(|flag| format_ident!("{}", flag.to_string().to_lowercase())).collect();

    let flag_idents: Vec<_> = lower_flags.iter().map(|flag| format_ident!("flag_{}", flag)).collect();
    let has_flags: Vec<_> = lower_flags.iter().map(|flag| format_ident!("has_{}", flag)).collect();
    let toggle_flags: Vec<_> = lower_flags.iter().map(|flag| format_ident!("toggle_{}", flag)).collect();
    let insert_flags: Vec<_> = lower_flags.iter().map(|flag| format_ident!("insert_{}", flag)).collect();
    let remove_flags: Vec<_> = lower_flags.iter().map(|flag| format_ident!("remove_{}", flag)).collect();


    TokenStream::from(quote!(
        #[napi]
        #[derive(Clone)]
        pub struct #name {
            #(pub(crate) #flag_idents: bool),*
        }

        impl From<#origin_ident> for #name {
            fn from(origin: #origin_ident) -> Self {
                #(
                    let #flag_idents = origin.contains(#origin_ident::#flags);
                )*
                Self { #( #flag_idents ),* }
            }
        }

        impl Into<#origin_ident> for #name {
            fn into(self) -> #origin_ident {
                let mut origin = #origin_ident::empty();
                let Self { #( #flag_idents ),* } = self;
                #(
                    if #flag_idents { origin.insert(#origin_ident::#flags) }
                )*
                origin
            }
        }

        #[napi]
        impl #name {
            #[napi(factory)]
            pub fn all() -> Self {
                Self { #( #flag_idents: true ),* }
            }
            #[napi(factory)]
            pub fn empty() -> Self {
                Self { #( #flag_idents: false ),* }
            }
            #[napi]
            pub fn is_all(&self) -> bool {
                let Self { #( #flag_idents ),* } = self;
                true #( && *#flag_idents )*
            }
            #[napi]
            pub fn is_empty(&self) -> bool {
                let Self { #( #flag_idents ),* } = self;
                !(false #( || *#flag_idents )*)
            }

            #(
                #[napi]
                pub fn #has_flags(&self) -> bool {
                    self.#flag_idents
                }
            )*
            #(
                #[napi(ts_return_type="this")]
                pub fn #toggle_flags<'a>(&mut self, this: This<'a, JsObject>) -> This<'a, JsObject> {
                    self.#flag_idents = !self.#flag_idents;
                    this
                }
            )*
            #(
                #[napi(ts_return_type="this")]
                pub fn #insert_flags<'a>(&mut self, this: This<'a, JsObject>) -> This<'a, JsObject> {
                    self.#flag_idents = true;
                    this
                }
            )*
            #(
                #[napi(ts_return_type="this")]
                pub fn #remove_flags<'a>(&mut self, this: This<'a, JsObject>) -> This<'a, JsObject> {
                    self.#flag_idents = false;
                    this
                }
            )*
        }
    ))
}