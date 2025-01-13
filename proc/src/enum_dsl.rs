use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Field, Fields, FieldsNamed, FieldsUnnamed, ItemEnum};

pub fn enum_dsl(input: TokenStream) -> TokenStream {
    let ItemEnum { ident, variants, .. } = parse_macro_input!(input as ItemEnum);

    let type_ident = format_ident!("{}Type", ident);

    let type_enum: Vec<_> = variants.iter()
        .map(|v| v.ident.clone())
        .collect();

    let sub_idents: Vec<_> = variants.iter()
        .map(|v| format_ident!("{}_{}", ident, v.ident))
        .collect();

    let type_quote = quote!(
        // #[napi]
        pub enum #type_ident {
            #(#type_enum),*
        }
    );

    let js_ident = format_ident!("Js{}", ident);

    let js = quote!(
        // #[napi]
        pub struct #js_ident {
            #(#type_enum: #sub_idents),*
        }
    );

    let sub_struct: Vec<_> = variants.iter()
        .map(|v| {
            let sub_ident = format_ident!("{}_{}", ident, v.ident);
            let sub_fields = match v.fields {
                Fields::Named(FieldsNamed { ref named, .. }) => {
                    named.iter().map(Clone::clone).collect()
                },
                Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => {
                    (0..unnamed.len()).into_iter().map(|i| {
                        // format!("_{:02}", i);
                        let field_ident = format_ident!("_{:02}", i);
                        let unnamed_field = unnamed.get(i).unwrap().clone();

                        Field { ident: Some(field_ident), ..unnamed_field }
                    }).collect()
                },
                Fields::Unit => vec![],
            };

            quote!(
                // #[napi]
                pub struct #sub_ident {
                    #(#sub_fields),*
                }
            )
        })
        .collect();

    let final_code = quote! {
        #type_quote
        #(#sub_struct)*
        #js
    };

    dbg!(final_code.to_string());

    TokenStream::from(final_code)
}