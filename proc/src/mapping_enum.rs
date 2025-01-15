#![allow(unused_variables)]

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, parse_str, Expr, Field, Fields, FieldsNamed, FieldsUnnamed, ItemEnum, Meta, MetaNameValue, Type, TypePath, Variant};

use case::CaseExt;
use crate::utils::to_lit_str;

const DIRCT_TYPES_STR: &str = "(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, f32, f64, usize, isize, bool)";

pub fn mapping_enum(input: TokenStream) -> TokenStream {
    let ItemEnum { attrs, ident, variants, generics, .. } = parse_macro_input!(input as ItemEnum);

    let conf_skip_type_name = attrs
        .iter()
        .filter(|attr| matches!(attr.path().get_ident(), Some(ident) if ident == "conf_skip_type_name"))
        .count();

    let Ok(Type::Tuple(type_tuple)) = parse_str::<Type>(DIRCT_TYPES_STR) else {
        unreachable!("direct_type can only be derived for tuples");
    };
    let direct_type_all: Vec<Type> = type_tuple.elems.iter().map(Clone::clone).collect();

    let enum_ident = ident.clone();
    let enum_ident_lit = to_lit_str(Box::new(enum_ident.clone()));

    let mapped_ident = format_ident!("Origin{}", enum_ident);

    let mod_ident = format_ident!("js_{}", enum_ident.to_string().to_snake());

    let enum_type_enum = format_ident!("{}Enum", enum_ident);
    let enum_type_variant_idents: Vec<_> = variants.iter()
        .map(|v| v.ident.clone())
        .collect();

    let variant_ignore_inners_full: Vec<_> = variants.iter()
        .map(|v| ignore_inner(v))
        .collect();

    let as_struct_variants: Vec<_> = variants.iter()
        .filter(|v| !matches!(v.fields, Fields::Unit))
        .collect();
    let as_struct_enum_type_idents: Vec<_> = as_struct_variants.iter()
        .map(|v| v.ident.clone())
        .collect();

    let struct_idents: Vec<_> = as_struct_variants.iter()
        .map(|v| format_ident!("{}{}Spec", ident, v.ident.clone()))
        .collect();

    let get_field_idents: Vec<_> = as_struct_variants.iter()
        .map(|v| format_ident!("get_{}", v.ident.to_string().to_snake()))
        .collect();

    let struct_ident_lit_strs: Vec<_> = as_struct_variants.iter()
        .map(|v| to_lit_str(Box::new(v.ident.clone())))
        .collect();

    let variant_ignore_inners: Vec<_> = as_struct_variants.iter()
        .map(|v| ignore_inner(v))
        .collect();

    let spec_fields_group: Vec<_> = as_struct_variants.iter()
        .map(|v| {
            // v.attrs;
            let fields = match v.fields {
                Fields::Named(FieldsNamed { ref named, .. }) => named.iter().map(Clone::clone).collect(),
                Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => (0..unnamed.len())
                    .into_iter()
                    .map(|i| {
                        let local_ident = format_ident!("elem{}", i);
                        let f = unnamed.get(i).unwrap().clone();

                        Field { ident: Some(local_ident), ..f }
                    })
                    .collect(),
                Fields::Unit => vec![],
            };

            let spec_fields: Vec<_> = fields.into_iter().map(|f| {
                let Field { attrs, ty, .. } = f.clone();

                let cleared_attrs: Vec<_> = attrs.iter()
                    .filter(|attr| {
                        let path_name = attr.meta.path().into_token_stream().to_string();
                        path_name != "conf_direct_type" && path_name != "conf_trans_type"
                    })
                    .map(Clone::clone)
                    .collect();

                if direct_type_all.contains(&ty) {
                    return (ty, Field { attrs: cleared_attrs, ..f });
                }

                let conf_direct_type = attrs.iter().find(
                    |attr| attr.meta.path().into_token_stream().to_string() == "conf_direct_type"
                );

                if let Some(_) = conf_direct_type {
                    return (ty, Field { attrs: cleared_attrs, ..f });
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

                    return (ty, Field { attrs: cleared_attrs, ty: trans_ty, ..f });
                }

                if let Type::Path(type_path) = ty.clone() {
                    let type_name = type_path.into_token_stream().to_string();

                    // let trans_ty = parse_str::<TypePath>(format!("Origin{}", type_name).as_str()).unwrap();
                    // let trans_ty = Type::Path(trans_ty);

                    return (ty, Field { attrs: cleared_attrs, ..f });
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

            spec_fields
        })
        .collect();

    let spec_fields_quotes: Vec<_> = spec_fields_group.iter().map(|fields| {
        let fields: Vec<_> = fields.iter().map(|(oty, f)| {
            let Field { ident, ty, .. } = f;

            quote!(pub(crate) #ident: #ty)
        }).collect();
        quote!(#( #fields ),*)
    }).collect();

    let spec_fields_getters: Vec<_> = spec_fields_group.iter().map(|fields| {
        let getters: Vec<_> = fields.iter().map(|(oty, f)| {
            let Field { ident, ty, .. } = f;

            let field_ident = ident.clone().unwrap();
            let field_ident_lit = to_lit_str(Box::new(field_ident.clone()));
            let field_getter_ident = format_ident!("get_{}", field_ident);

            quote!(
                // #[napi(getter, js_name = #field_ident_lit)]
                pub fn #field_getter_ident(&self) -> #ty {
                    self.#ident.ex_into()
                }
            )
        }).collect();

        quote!(#( #getters )*)
    }).collect();

    let sub_fields_from_inner_reaches: Vec<_> = as_struct_variants.iter()
        .map(|v| {
            let variant_ident = &v.ident;
            let reach = quote! { #mapped_ident::#variant_ident };
            // v.attrs;
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

    let get_type_name = if conf_skip_type_name <= 0 { quote!(
        // #[napi(getter, js_name = "typeName")]
        pub fn get_type_name(&self) -> String {
            match self.inner {
                #( #mapped_ident::#enum_type_variant_idents #variant_ignore_inners_full => String::from(stringify!(#enum_type_variant_idents)) ),*
            }
        }
    ) } else { quote!() };

    TokenStream::from(quote!(
        // #[napi(string_enum)]
        pub enum #enum_type_enum {
            #(#enum_type_variant_idents),*
        }

        #(
            // #[napi]
            pub struct #struct_idents {
                #spec_fields_quotes
            }

            impl From<#mapped_ident #generics> for #struct_idents {
                fn from(value: #mapped_ident #generics) -> Self {
                    match value {
                        #sub_fields_from_inner_reaches,
                        _ => unreachable!("no matched convert")
                        // _ => unreachable!(format!("cannot convert [{}] from [{}]", stringify!(#struct_idents), value)),
                    }
                }
            }

            // #[napi]
            impl #struct_idents {
                #spec_fields_getters
            }
        )*

        // #[napi]
        pub struct #enum_ident {
            pub(crate) inner: #mapped_ident #generics,
        }

        // #[napi]
        impl #enum_ident {
            // #[napi(getter, js_name = "type")]
            pub fn get_type(&self) -> #enum_type_enum {
                match self.inner {
                    #( #mapped_ident::#enum_type_variant_idents #variant_ignore_inners_full => #enum_type_enum::#enum_type_variant_idents ),*
                }
            }

            #get_type_name

            #(
                // #[napi(getter, js_name = #variant_idents_lit)]
                pub fn #get_field_idents(&self) -> Result<#struct_idents> {
                    match &self.inner {
                        #mapped_ident::#as_struct_enum_type_idents #variant_ignore_inners => Ok(#struct_idents::ex_from(self.inner.clone())),
                        _ => Err(napi::Error::from_reason(format!("cannot access [{}] when type is [{}]", stringify!(#struct_idents), self.get_type_name()))),
                    }
                }
            )*
        }
    ))
}

#[inline]
fn ignore_inner(v: &Variant) -> proc_macro2::TokenStream {
    match v.fields {
        Fields::Named(_) => quote!({ .. }),
        Fields::Unnamed(_) => quote!((_)),
        Fields::Unit => quote!(),
    }
}

#[allow(dead_code)]
#[inline]
fn ty_to_string(ty: &Type) -> String {
    ty.into_token_stream().to_string()
}