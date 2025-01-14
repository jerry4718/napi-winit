use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, parse_str, Expr, Field, Fields, FieldsNamed, FieldsUnnamed, ItemEnum, LitStr, Meta, MetaNameValue, Type, TypePath};

use case::CaseExt;

const DIRCT_TYPES_STR: &str = "(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64, usize, isize, bool)";

pub fn mapping_enum(input: TokenStream) -> TokenStream {
    let ItemEnum { ident, variants, generics, .. } = parse_macro_input!(input as ItemEnum);

    let Ok(Type::Tuple(type_tuple)) = parse_str::<Type>(DIRCT_TYPES_STR) else {
        unreachable!("dirct_type can only be derived for tuples");
    };
    let dirct_type_all: Vec<Type> = type_tuple.elems.iter().map(Clone::clone).collect();

    let ident_lit = to_lit_str(Box::new(ident.clone()));

    let origin_ident = format_ident!("Origin{}", ident);

    let mod_ident = format_ident!("js_{}", ident.to_string().to_snake());

    let get_field_idents: Vec<_> = variants.iter()
        .map(|v| format_ident!("get_{}", v.ident.to_string().to_snake()))
        .collect();

    let variant_idents: Vec<_> = variants.iter()
        .map(|v| v.ident.clone())
        .collect();

    let variant_idents_lit: Vec<_> = variants.iter()
        .map(|v| to_lit_str(Box::new(v.ident.clone())))
        .collect();

    let variant_ignore_inners: Vec<_> = variants.iter()
        .map(|v| match v.fields {
            Fields::Named(_) => quote!({ .. }),
            Fields::Unnamed(_) => quote!((_)),
            Fields::Unit => quote!(),
        })
        .collect();

    let sub_fields_quote: Vec<_> = variants.iter()
        .map(|v| {
            let sub_fields = match v.fields {
                Fields::Named(FieldsNamed { ref named, .. }) => named.iter().map(Clone::clone).collect(),
                Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => (0..unnamed.len())
                    .into_iter()
                    .map(|i| {
                        let local_ident = format_ident!("elem{}", i);
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
                let Field { attrs, ty, .. } = f.clone();

                let cleared_attrs: Vec<_> = attrs.iter()
                    .filter(|attr| {
                        let path_name = attr.meta.path().into_token_stream().to_string();
                        path_name != "conf_dirct_type" && path_name != "conf_trans_type"
                    })
                    .map(Clone::clone)
                    .collect();

                if dirct_type_all.contains(&ty) {
                    return Field { attrs: cleared_attrs, ..f };
                }

                let conf_dirct_type = attrs.iter().find(
                    |attr| attr.meta.path().into_token_stream().to_string() == "conf_dirct_type"
                );

                if let Some(_) = conf_dirct_type {
                    // find_origin.insert(ty.clone(), ty.clone());
                    // find_trans.insert(ty.clone(), ty.clone());
                    return Field { attrs: cleared_attrs, ..f };
                }

                let conf_trans_type = attrs.iter().find(
                    |attr| attr.meta.path().into_token_stream().to_string() == "conf_trans_type"
                );

                if let Some(conf_trans_type) = conf_trans_type {
                    let Meta::NameValue(MetaNameValue { ref value, .. }) = conf_trans_type.meta
                    else { panic!("#[conf_trans_type = xxx] that xxx must be specified") };

                    let Expr::Path(path) = value
                    else { panic!("#[conf_trans_type = xxx] that xxx must be an simple type") };

                    let trans_ty = syn::parse::<TypePath>(TokenStream::from(path.into_token_stream())).unwrap();
                    let trans_ty = Type::Path(trans_ty);

                    // find_origin.insert(trans_ty.clone(), ty.clone());
                    // find_trans.insert(ty.clone(), trans_ty.clone());

                    return Field { attrs: cleared_attrs, ty: trans_ty, ..f };
                }

                if let Type::Path(type_path) = ty.clone() {
                    let type_name = type_path.into_token_stream().to_string();

                    let trans_ty = parse_str::<TypePath>(format!("Origin{}", type_name).as_str()).unwrap();
                    let trans_ty = Type::Path(trans_ty);

                    // find_origin.insert(trans_ty.clone(), ty.clone());
                    // find_trans.insert(ty.clone(), trans_ty.clone());

                    return Field { attrs: cleared_attrs, ty: trans_ty, ..f };
                }

                match ty {
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

                    quote!(#reach { #(#fields),* } => Self { #(#fields: #fields.ex_into()),* })
                }
                Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => {
                    let fields: Vec<_> = (0..unnamed.len())
                        .into_iter()
                        .map(|i| format_ident!("elem{}", i))
                        .collect();

                    quote!(#reach( #(#fields),* ) => Self { #(#fields: #fields.ex_into()),* })
                }
                Fields::Unit => quote!(#reach => Self { }),
            }
        })
        .collect();

    TokenStream::from(quote!(
        // #[napi(js_name = #ident_lit)]
        pub mod #mod_ident {
            use super::*;

            // #[napi]
            pub enum Type {
                #(#variant_idents),*
            }

            #(
                // #[napi]
                pub struct #variant_idents {
                    #sub_fields_quote
                }

                impl From<#origin_ident #generics> for #variant_idents {
                    fn from(value: #origin_ident #generics) -> Self {
                        match value {
                            #sub_fields_from_origin_reaches,
                            _ => unreachable!(" i hope "),
                        }
                    }
                }

                // #[napi]
                impl #variant_idents {
                    // pub fn #sub_fields_quote() {
                    //
                    // }
                }
            )*

            // #[napi]
            pub struct #ident {
                pub(crate) inner: #origin_ident #generics,
            }

            // #[napi]
            impl #ident {
                // #[napi(getter, js_name = "type")]
                pub fn get_type(&self) -> Type {
                    match self.inner {
                        #( #origin_ident::#variant_idents #variant_ignore_inners => Type::#variant_idents ),*
                    }
                }

                // #[napi(getter, js_name = "typeName")]
                pub fn get_type_name(&self) -> String {
                    match self.inner {
                        #( #origin_ident::#variant_idents #variant_ignore_inners => String::from(stringify!(#variant_idents)) ),*
                    }
                }

                #(
                    // #[napi(getter, js_name = #variant_idents_lit)]
                    pub fn #get_field_idents(&self) -> Result<#variant_idents> {
                        match &self.inner {
                            #origin_ident::#variant_idents #variant_ignore_inners => Ok(#variant_idents::ex_from(self.inner.clone())),
                            _ => Err(napi::Error::from_reason(format!("cannot access [{}] when type is [{}]", stringify!(#variant_idents), self.get_type_name()))),
                        }
                    }
                )*
            }
        }
    ))
}

#[allow(dead_code)]
const EQ_TYPE: fn(&Type, &Type) -> bool = <Type as PartialEq<Type>>::eq;

#[inline]
fn to_lit_str(tokens: Box<dyn ToTokens>) -> LitStr {
    let string = tokens.into_token_stream().to_string();
    let inner = vec![String::from('"'), string, String::from('"')];
    parse_str::<LitStr>(&inner.join("")).unwrap()
}

#[allow(dead_code)]
#[inline]
fn ty_to_string(ty: &Type) -> String {
    ty.into_token_stream().to_string()
}