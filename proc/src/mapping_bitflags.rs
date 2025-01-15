use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Ident, Token};
use syn::parse::{Parse, ParseStream};
use crate::utils::to_lit_str;

struct FlagsInputSpec {
    name: Ident,
    flags: Vec<Ident>
}

impl Parse for FlagsInputSpec {
    #[rustfmt::skip]
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let _token_colon = input.parse::<Token![:]>()?;

        let mut flags = Vec::<Ident>::new();

        // 形参列表：开始
        while let Some(flag) = input.parse::<Option<Ident>>()? {
            input.parse::<Option<Token![;]>>()?;
            flags.push(flag);
        }
        Ok(Self { name, flags })
    }
}

pub fn mapping_bitflags(input: TokenStream) -> TokenStream {
    let FlagsInputSpec { name, flags } = parse_macro_input!(input as FlagsInputSpec);

    let origin_ident = format_ident!("Origin{}", name);
    let origin_ident_lit = to_lit_str(Box::new(origin_ident.clone()));

    let lower_flags: Vec<_> = flags.iter().map(|flag| format_ident!("flag_{}", flag.to_string().to_lowercase())).collect();

    let has_flags: Vec<_> = lower_flags.iter().map(|flag| format_ident!("has_{}", flag)).collect();
    let toggle_flags: Vec<_> = lower_flags.iter().map(|flag| format_ident!("toggle_{}", flag)).collect();
    let insert_flags: Vec<_> = lower_flags.iter().map(|flag| format_ident!("insert_{}", flag)).collect();
    let remove_flags: Vec<_> = lower_flags.iter().map(|flag| format_ident!("remove_{}", flag)).collect();


    TokenStream::from(quote!(
        #[napi(js_name = #origin_ident_lit)]
        #[derive(Copy, Clone)]
        pub struct #name {
            #(pub(crate) #lower_flags: bool),*
        }

        impl Into<#origin_ident> for #name {
            fn into(self) -> #origin_ident {
                let mut origin = #origin_ident::empty();
                let Self { #( #lower_flags ),* } = self;
                #(
                    if #lower_flags { origin.insert(#origin_ident::#flags) }
                )*
                origin
            }
        }

        #[napi]
        impl #name {
            #[napi(factory)]
            pub fn all() -> Self {
                Self { #( #lower_flags: true ),* }
            }
            #[napi(factory)]
            pub fn empty() -> Self {
                Self { #( #lower_flags: false ),* }
            }
            #[napi]
            pub fn is_all(&self) -> bool {
                let Self { #( #lower_flags ),* } = self;
                true #( && *#lower_flags )*
            }
            #[napi]
            pub fn is_empty(&self) -> bool {
                let Self { #( #lower_flags ),* } = self;
                !(false #( || *#lower_flags )*)
            }

            #(
                pub fn #has_flags(&self) -> bool {
                    self.#lower_flags
                }

                #[napi(ts_return_type="this")]
                pub fn #toggle_flags(&mut self, this: This<JsObject>) -> This<JsObject> {
                    self.#lower_flags = !self.#lower_flags;
                    this
                }

                #[napi(ts_return_type="this")]
                pub fn #insert_flags(&mut self, this: This<JsObject>) -> This<JsObject> {
                    self.#lower_flags = true;
                    this
                }

                #[napi(ts_return_type="this")]
                pub fn #remove_flags(&mut self, this: This<JsObject>) -> This<JsObject> {
                    self.#lower_flags = false;
                    this
                }
            )*
        }
    ))
}