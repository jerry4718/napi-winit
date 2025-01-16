use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Token, Type};

struct SimpleStructInputSpec {
    name: Ident,
    origin: Option<Type>
}

impl Parse for SimpleStructInputSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        let token_colon = input.parse::<Option<Token![:]>>()?;

        if token_colon.is_none() {
            return Ok(Self { name, origin: None })
        }

        let origin = input.parse::<Type>()?;

        Ok(Self { name, origin: Some(origin) })
    }
}

pub fn simple_struct(input: TokenStream) -> TokenStream {
    let SimpleStructInputSpec { name, origin } = parse_macro_input!(input as SimpleStructInputSpec);

    let origin = match origin {
        None => {
            syn::parse_str::<Type>(format!("Origin{}", name.to_string()).as_str()).unwrap()
        },
        Some(origin) => origin
    };

    TokenStream::from(quote!(
        #[napi]
        #[derive(Clone)]
        pub struct #name(pub(crate) #origin);

        impl From<#origin> for #name {
            fn from(origin: #origin) -> Self {
                Self(origin)
            }
        }

        impl Into<#origin> for #name {
            fn into(self) -> #origin {
                self.0
            }
        }
    ))
}