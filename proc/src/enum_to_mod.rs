use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use std::collections::HashMap;
use std::str::FromStr;
use syn::{parse_macro_input, Expr, Field, Fields, FieldsNamed, FieldsUnnamed, ItemEnum, Meta, MetaNameValue, Type, TypePath};

use case::CaseExt;

pub fn enum_to_mod(input: TokenStream) -> TokenStream {
    let ItemEnum { ident, variants, .. } = parse_macro_input!(input as ItemEnum);

    let mut find_origin = HashMap::<String, Type>::new();
    let mut find_trans = HashMap::<String, Type>::new();

    let origin_ident = format_ident!("Origin{}", ident);
    let mod_ident = format_ident!("js_{}", ident.to_string().to_snake());

    let get_field_idents: Vec<_> = variants.iter()
        .map(|v| format_ident!("get_{}", v.ident.to_string().to_snake()))
        .collect();

    let variant_idents: Vec<_> = variants.iter()
        .map(|v| v.ident.clone())
        .collect();

    let variant_ignore_inners: Vec<_> = variants.iter()
        .map(|v| match v.fields {
            Fields::Named(_) => quote!({ .. }),
            Fields::Unnamed(_) => quote!((_)),
            Fields::Unit => quote!(),
        })
        .collect();

    let type_quote = quote!(
        // #[napi]
        pub enum Type {
            #(#variant_idents),*
        }
    );

    let face_quote = quote!(
        // #[napi]
        pub struct #ident {
            pub(crate) inner: #origin_ident,
        }

        impl #ident {
            pub fn get_type(&self) -> Type {
                match self.inner {
                    #( #origin_ident::#variant_idents #variant_ignore_inners => Type::#variant_idents ),*
                }
            }
            pub fn get_type_name(&self) -> String {
                match self.inner {
                    #( #origin_ident::#variant_idents #variant_ignore_inners => String::from(stringify!(#variant_idents)) ),*
                }
            }
            #(
                pub fn #get_field_idents(&self) -> Result<#variant_idents> {
                    match &self.inner {
                        #origin_ident::#variant_idents #variant_ignore_inners => Ok(#variant_idents::from(self.inner.clone())),
                        _ => Err(napi::Error::from_reason("cannot access [{}] when type is [{}]", stringify!(#variant_idents), self.get_type_name())),
                    }
                }
            )*
        }
    );

    let sub_fields_quote: Vec<_> = variants.iter()
        .map(|v| {
            let sub_fields = match v.fields {
                Fields::Named(FieldsNamed { ref named, .. }) => named.iter().map(Clone::clone).collect(),
                Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => (0..unnamed.len())
                    .into_iter()
                    .map(|i| {
                        let local_ident = format_ident!("unnamed{:02}", i);
                        let unnamed_field = unnamed.get(i).unwrap().clone();

                        Field {
                            ident: Some(local_ident),
                            ..unnamed_field
                        }
                    })
                    .collect(),
                Fields::Unit => vec![],
            };

            let sub_fields: Vec<_> = sub_fields.into_iter().map(|f| {
                let Field { attrs, .. } = f.clone();

                let cleared_attrs: Vec<_> = attrs.iter()
                    .filter(|attr| {
                        let path_name = attr.meta.path().into_token_stream().to_string();
                        path_name != "conf_dirct_type" && path_name != "conf_trans_type"
                    })
                    .map(Clone::clone)
                    .collect();

                let conf_dirct_type = attrs.iter().find(
                    |attr| attr.meta.path().into_token_stream().to_string() == "conf_dirct_type"
                );

                if let Some(_) = conf_dirct_type {
                    find_origin.insert(ty_to_string(&f.ty), f.ty.clone());
                    find_trans.insert(ty_to_string(&f.ty), f.ty.clone());
                    return Field { attrs: cleared_attrs, ..f };
                }

                let conf_trans_type = attrs.iter().find(
                    |attr| attr.meta.path().into_token_stream().to_string() == "conf_trans_type"
                );

                if let Some(conf_trans_type) = conf_trans_type {
                    let Meta::NameValue(MetaNameValue { ref value, .. }) = conf_trans_type.meta
                    else { panic!("#[conf_trans_type = xxx] that xxx must be specified") };

                    let Expr::Path(ty) = value
                    else { panic!("#[conf_trans_type = xxx] that xxx must be an simple type") };

                    let trans_ty = syn::parse::<TypePath>(TokenStream::from(ty.into_token_stream())).unwrap();
                    let trans_ty = Type::Path(trans_ty);

                    find_origin.insert(ty_to_string(&trans_ty), f.ty.clone());
                    find_trans.insert(ty_to_string(&f.ty), trans_ty.clone());

                    return Field { attrs: cleared_attrs, ty: trans_ty, ..f };
                }

                if let Type::Path(type_path) = f.ty.clone() {
                    let type_name = type_path.into_token_stream().to_string();
                    let origin = format!("Origin{}", type_name);

                    let trans_ty = syn::parse::<TypePath>(TokenStream::from_str(&*origin).unwrap()).unwrap();
                    let trans_ty = Type::Path(trans_ty);

                    find_origin.insert(ty_to_string(&trans_ty), f.ty.clone());
                    find_trans.insert(ty_to_string(&f.ty), trans_ty.clone());

                    return Field { attrs: cleared_attrs, ty: trans_ty, ..f };
                }

                match f.ty {
                    Type::Array(_) => println!("Type::Array"),
                    Type::BareFn(_) => println!("Type::BareFn"),
                    Type::Group(_) => println!("Type::Group"),
                    Type::ImplTrait(_) => println!("Type::ImplTrait"),
                    Type::Infer(_) => println!("Type::Infer"),
                    Type::Macro(_) => println!("Type::Macro"),
                    Type::Never(_) => println!("Type::Never"),
                    Type::Paren(_) => println!("Type::Paren"),
                    Type::Path(_) => println!("Type::Path"),
                    Type::Ptr(_) => println!("Type::Ptr"),
                    Type::Reference(_) => println!("Type::Reference"),
                    Type::Slice(_) => println!("Type::Slice"),
                    Type::TraitObject(_) => println!("Type::TraitObject"),
                    Type::Tuple(_) => println!("Type::Tuple"),
                    Type::Verbatim(_) => println!("Type::Verbatim"),
                    _ => {}
                }

                unimplemented!("here")
            }).collect();

            quote!(#(#sub_fields),*)
        })
        .collect();

    let sub_fields_from_origin_reaches: Vec<_> = variants.iter()
        .map(|v| {
            let enum_ident = &v.ident;
            let reach = quote! { #origin_ident::#enum_ident };
            match v.fields {
                Fields::Named(FieldsNamed { ref named, .. }) => {
                    let fields: Vec<_> = named.iter().map(|f| f.ident.clone().unwrap()).collect();

                    quote!(#reach { #(#fields),* } => Self { #(#fields: #fields.into()),* })
                }
                Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => {
                    let fields: Vec<_> = (0..unnamed.len())
                        .into_iter()
                        .map(|i| format_ident!("unnamed{:02}", i))
                        .collect();

                    quote!(#reach( #(#fields),* ) => Self { #(#fields: #fields.into()),* })
                }
                Fields::Unit => quote!(#reach => Self { }),
            }
        })
        .collect();

    TokenStream::from(quote!(
      pub mod #mod_ident {
          use super::*;

          #type_quote
          #(
              pub struct #variant_idents {
                  #sub_fields_quote
              }
          )*
          #(
              impl From<#origin_ident> for #variant_idents {
                  fn from(value: #origin_ident) -> Self {
                      match value {
                          #sub_fields_from_origin_reaches,
                          _ => unreachable!(" i hope "),
                      }
                  }
              }
          )*
          #face_quote
      }
    ))
}

#[inline]
fn ty_to_string(ty: &Type) -> String {
    ty.into_token_stream().to_string()
}
