use crate::utils::{append_to_tokens, get_meta_by_name, get_meta_value_as_expr_tuple, get_type_ty_or, parse_as, parse_metas};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, Ident, ItemStruct, Type};

pub(crate) fn proxy_flags(attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let metas = parse_metas(attrs);
    let input = parse_macro_input!(input as ItemStruct);

    let origin = get_type_ty_or(
        &get_meta_by_name(&metas, "origin"),
        &format_ident!("Origin{}", input.ident),
    );

    let bitflags = get_meta_by_name(&metas, "flags");

    let Some(flags_meta) = bitflags
    else { panic!("`flags` must assigned on #[proxy_flags]") };

    let Some(expr_tuple) = get_meta_value_as_expr_tuple(&flags_meta)
    else { panic!("value for `flags` must like a tuple (FLAG_A, FLAG_B, FLAG_C)") };

    let flags: Vec<_> = expr_tuple.elems.iter().map(|el| parse_as::<Ident>(el)).collect();

    let proxy_flags = ProxyFlags {
        input,
        origin,
        flags,
    };

    proc_macro::TokenStream::from(quote! { #proxy_flags })
}

struct ProxyFlags {
    pub input: ItemStruct,
    pub origin: Type,
    pub flags: Vec<Ident>,
}

impl ToTokens for ProxyFlags {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ProxyFlags {
            input: ItemStruct { ident: name, attrs, .. },
            origin: origin_ty,
            flags,
        } = self;

        let lower_names: Vec<_> = flags.iter().map(|flag| format_ident!("{}", flag.to_string().to_lowercase())).collect();
        let lower_names_zip: Vec<_> = flags.iter().zip(&lower_names).collect();

        macro_rules! spanned_from_flag_zip {
            ($local: ident, $ident: ident = $fmt: literal => $($tt: tt)*) => {
                let $local: Vec<TokenStream> = lower_names_zip.iter()
                    .map(|(flag, lower_flag)| {
                        let $ident = format_ident!($fmt, lower_flag);
                        quote_spanned! { flag.span() => $($tt)* }
                    })
                    .collect();
            };
        }

        spanned_from_flag_zip!(flag_idents, ident = "flag_{}" => #ident);

        let zip2: Vec<_> = flags.iter().zip(lower_names.iter().zip(&flag_idents)).collect();

        append_to_tokens(tokens, quote_spanned! { name.span() =>
            #[napi]
            #( #attrs )*
            pub struct #name {
                #(pub(crate) #flag_idents: bool),*
            }
        });

        let flag_dispose: Vec<_> = zip2.iter()
            .map(|(flag, (_, field_ident))| quote_spanned!( flag.span() =>
                let #field_ident = origin.contains(#origin_ty::#flag);
            ))
            .collect();

        append_to_tokens(tokens, quote_spanned! { name.span() =>
            impl From<#origin_ty> for #name {
                fn from(origin: #origin_ty) -> Self {
                    #( #flag_dispose )*
                    Self { #( #flag_idents ),* }
                }
            }
        });

        let flag_compose: Vec<_> = zip2.iter()
            .map(|(flag, (_, field_ident))| quote_spanned!( flag.span() =>
                if #field_ident { origin.insert(#origin_ty::#flag) }
            ))
            .collect();

        append_to_tokens(tokens, quote_spanned! { name.span() =>
            impl Into<#origin_ty> for #name {
                fn into(self) -> #origin_ty {
                    let mut origin = #origin_ty::empty();
                    let Self { #( #flag_idents ),* } = self;
                    #( #flag_compose )*
                    origin
                }
            }
        });

        let mut fns = TokenStream::default();

        append_to_tokens(&mut fns, quote! {
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
                true #( && !*#flag_idents )*
            }
        });

        spanned_from_flag_zip!(has_idents, ident = "has_{}" => #ident);
        append_to_tokens(&mut fns, quote! {
            #(
                #[napi]
                pub fn #has_idents(&self) -> bool {
                    self.#flag_idents
                }
            )*
        });

        spanned_from_flag_zip!(toggle_idents, ident = "toggle_{}" => #ident);
        append_to_tokens(&mut fns, quote! {
            #(
                #[napi(ts_return_type="this")]
                pub fn #toggle_idents(&mut self) -> &Self {
                    self.#flag_idents = !self.#flag_idents;
                    self
                }
            )*
        });

        spanned_from_flag_zip!(insert_idents, ident = "insert_{}" => #ident);
        append_to_tokens(&mut fns, quote! {
            #(
                #[napi(ts_return_type="this")]
                pub fn #insert_idents(&mut self) -> &Self {
                    self.#flag_idents = true;
                    self
                }
            )*
        });

        spanned_from_flag_zip!(remove_idents, ident = "remove_{}" => #ident);
        append_to_tokens(&mut fns, quote! {
            #(
                #[napi(ts_return_type="this")]
                pub fn #remove_idents(&mut self) -> &Self {
                    self.#flag_idents = false;
                    self
                }
            )*
        });

        append_to_tokens(tokens, quote_spanned! { name.span() =>
            #[napi]
            impl #name {
                #fns
            }
        });
    }
}