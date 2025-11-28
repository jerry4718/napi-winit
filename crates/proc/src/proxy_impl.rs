use crate::{
    conf_usage::{get_meta_value_as_conf_usage, quote_option_conf_usage},
    utils::{append_to_tokens, get_meta_value_as, get_metas_by_attr_name, parse_as, parse_metas, separate_attr_by_name},
};
use macros::{define_const_str, map_meta_to_local};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fmt::Debug;
use syn::{
    parse,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated, token::Pub,
    Attribute, Expr, FnArg, ImplItem, ImplItemFn, ItemImpl, Meta, Pat, PatIdent, PatType, Receiver, ReturnType, Signature, Token, TraitItemFn, Visibility,
};

pub(crate) fn proxy_impl(attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let metas = parse_metas(attrs);

    let item_impl = parse_macro_input!(input as ItemImpl);

    let proxy_impl = parse_proxy_impl(&metas, &item_impl);

    proc_macro::TokenStream::from(quote! { #proxy_impl })
}

pub(crate) struct ProxyImpl {
    pub input: ItemImpl,
    pub items: Vec<ProxyImplItem>,
    pub reserved_attrs: Vec<Attribute>,
    pub access_expr: Option<Expr>,
}

pub(crate) enum ProxyImplItem {
    TraitFnLike(TraitFnLike),
    ImplFn(ImplItemFn),
}

pub(crate) struct TraitFnLike {
    pub vis: Option<Visibility>,
    pub attrs: Vec<Attribute>,
    pub sig: Signature,
}

define_const_str!(ATTR_PROXY_IMPL = proxy_impl);
const ATTR_INCLUDES: &[&str] = &[ATTR_PROXY_IMPL];

define_const_str!(
    META_ACCESS_EXPR = access_expr,
);

define_const_str!(
    META_CONV_ARG = conv_arg,
    META_SKIP_CONV_ARG = skip_conv_arg,
    META_CONV_RETURN = conv_return,
    META_SKIP_CONV_RETURN = skip_conv_return,
);

impl Parse for TraitFnLike {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)
            .unwrap_or_else(|e| Vec::new());

        let vis = input.parse::<Visibility>().ok();
        let TraitItemFn { sig, .. } = input.parse::<TraitItemFn>()?;

        Ok(Self { vis, attrs, sig })
    }
}

fn parse_proxy_impl(metas: &Vec<Meta>, item_impl: &ItemImpl) -> ProxyImpl {
    let ItemImpl { attrs, items, .. } = item_impl;

    let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);
    if matches!(matched.len(), n if n > 0) {
        panic!("so many proxy_impl 0");
    }

    let impl_items: Vec<ProxyImplItem> = items
        .iter()
        .map(parse_proxy_impl_item)
        .collect();

    map_meta_to_local!(&metas => {
        META_ACCESS_EXPR => access_expr,
    });

    ProxyImpl {
        input: item_impl.clone(),
        reserved_attrs: surplus,
        items: impl_items,
        access_expr: access_expr.as_ref().map(get_meta_value_as).flatten(),
    }
}

fn parse_proxy_impl_item(item: &ImplItem) -> ProxyImplItem {
    match item {
        ImplItem::Verbatim(payload) => {
            let trait_item_like = parse::<TraitFnLike>(proc_macro::TokenStream::from(payload.to_token_stream()))
                .expect("proxy_impl");

            ProxyImplItem::TraitFnLike(trait_item_like)
        }
        ImplItem::Const(_) => unimplemented!("Impl item: ImplItem::Const"),
        ImplItem::Fn(_) => unimplemented!("Impl item: ImplItem::Fn"),
        ImplItem::Type(_) => unimplemented!("Impl item: ImplItem::Type"),
        ImplItem::Macro(_) => unimplemented!("Impl item: ImplItem::Macro"),
        _ => unimplemented!("Impl item: NonExhaustive"),
    }
}

fn quote_proxy_impl_item(body: &ProxyImpl, item: &ProxyImplItem) -> TokenStream {
    let attrs = match item {
        ProxyImplItem::TraitFnLike(TraitFnLike { attrs, .. }) => attrs,
        _ => unimplemented!("on quote_proxy_impl_item"),
    };

    let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);

    map_meta_to_local!(&get_metas_by_attr_name(&matched, ATTR_PROXY_IMPL) => {
        META_ACCESS_EXPR => access_expr,
        META_CONV_RETURN => conv_return,
    });

    let conv_return = conv_return.as_ref()
        .map(get_meta_value_as_conf_usage).flatten();

    let access_expr = access_expr.as_ref()
        .map(get_meta_value_as)
        .flatten()
        .or(body.access_expr.clone())
        .unwrap_or_else(|| parse_as::<Expr>(&"self.inner"));

    match item {
        ProxyImplItem::TraitFnLike(TraitFnLike { vis, attrs, sig }) => {
            let vis = match vis {
                None | Some(Visibility::Inherited) => &Visibility::Public(Pub::default()),
                Some(vis) => vis,
            };
            let Signature { ident, inputs, output, .. } = sig;
            let invoke_inputs = inputs.iter()
                .filter(|input| !matches!(input, &FnArg::Receiver(_)))
                .map(|arg| {
                    match arg {
                        FnArg::Receiver(receiver) => unimplemented!("FnArg::Receiver({})", receiver.to_token_stream().to_string()),
                        FnArg::Typed(PatType { attrs, pat, ty, .. }) => {
                            let (matched, _) = separate_attr_by_name(attrs, ATTR_INCLUDES);

                            map_meta_to_local!(&get_metas_by_attr_name(&matched, ATTR_PROXY_IMPL) => {
                                META_CONV_ARG => conv_arg,
                                META_SKIP_CONV_ARG => skip_conv_arg,
                            });

                            let name = match pat.as_ref() {
                                Pat::Ident(PatIdent { ident, .. }) => ident,
                                _ => unimplemented!("on quote_proxy_impl_item"),
                            };

                            if skip_conv_arg.is_some() {
                                return quote! { #name };
                            }

                            let conv_arg = conv_arg.map(|meta| get_meta_value_as_conf_usage(&meta)).flatten();

                            quote_option_conf_usage(name, &conv_arg)
                        }
                    }
                })
                .collect::<Vec<_>>();

            let stmt = quote! { #access_expr.#ident(#( #invoke_inputs ),*) };

            let stmt = match output {
                ReturnType::Default => stmt,
                ReturnType::Type(_, _) => quote_option_conf_usage(&stmt, &conv_return),
            };

            let proxy_inputs = inputs.iter()
                .map(|fn_arg| {
                    match fn_arg {
                        FnArg::Receiver(receiver) => {
                            let (_, surplus) = separate_attr_by_name(&receiver.attrs, ATTR_INCLUDES);
                            FnArg::Receiver(Receiver {
                                attrs: surplus,
                                ..receiver.clone()
                            })
                        }
                        FnArg::Typed(pat_type) => {
                            let (_, surplus) = separate_attr_by_name(&pat_type.attrs, ATTR_INCLUDES);
                            FnArg::Typed(PatType {
                                attrs: surplus,
                                ..pat_type.clone()
                            })
                        }
                    }
                })
                .collect::<Vec<_>>();

            let sig = Signature {
                inputs: Punctuated::<FnArg, Token![,]>::from_iter(proxy_inputs),
                ..sig.clone()
            };

            let napi_metas = Vec::<TokenStream>::new();

            quote! {
                #( #surplus )*
                #[napi( #( #napi_metas )* )]
                #vis #sig {
                    #stmt
                }
            }
        }
        _ => unimplemented!("on quote_proxy_impl_item"),
    }
}

impl ToTokens for ProxyImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { input, items, reserved_attrs, .. } = self;

        let ItemImpl { self_ty, .. } = input;

        let napi_metas = Vec::<TokenStream>::new();

        let items: Vec<_> = items.iter().map(|item| quote_proxy_impl_item(self, item)).collect();

        append_to_tokens(tokens, quote! {
            #( #reserved_attrs )*
            #[napi( #( #napi_metas )* )]
            impl #self_ty {
                #( #items )*
            }
        });
    }
}