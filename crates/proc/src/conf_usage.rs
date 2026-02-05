use proc_macro2::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use syn::{Expr, ExprBlock, ExprCall, ExprClosure, ExprMethodCall, ExprPath, Meta, MetaNameValue};
use syn::spanned::Spanned;

pub(crate) enum ConfUsage {
    Path(ExprPath),
    Closure(ExprClosure),
    Block(ExprBlock),
    MethodCall(ExprMethodCall),
    Pipe(Vec<ConfUsage>),
    Call(ExprCall),
}

pub(crate) fn quote_option_conf_usage(from: &dyn ToTokens, option: &Option<ConfUsage>) -> TokenStream {
    match option {
        Some(conf_usage) => quote_conf_usage(from, conf_usage),
        None => quote! { #from.into() },
    }
}

pub(crate) fn quote_conf_usage(from: &dyn ToTokens, conf_usage: &ConfUsage) -> TokenStream {
    match conf_usage {
        ConfUsage::Path(path) => quote_spanned! { path.span() => #path(#from) },
        ConfUsage::Closure(closure) => quote_spanned! { closure.span() => ((#closure)(#from)) },
        ConfUsage::MethodCall(method_call) => quote_spanned! { method_call.span() => #method_call },
        ConfUsage::Block(block) => quote_spanned! { block.span() => #block },
        ConfUsage::Call(call) => quote_spanned! { call.span() => #call(#from) },
        ConfUsage::Pipe(usages) => {
            usages.iter()
                .fold(from.to_token_stream(), |ts, usage| quote_conf_usage(&ts, usage))
        }
    }
}

pub(crate) fn get_meta_value_as_conf_usage(meta: &Meta) -> Option<ConfUsage> {
    let Meta::NameValue(MetaNameValue { value, .. }) = meta
    else { return None };

    Some(expr_to_conf_usage(value))
}

fn expr_to_conf_usage(value: &Expr) -> ConfUsage {
    match value {
        Expr::Path(path) => ConfUsage::Path(path.clone()),
        Expr::Closure(closure) => ConfUsage::Closure(closure.clone()),
        Expr::Array(array) => ConfUsage::Pipe(array.elems.iter().map(expr_to_conf_usage).collect()),
        Expr::MethodCall(method_call) => ConfUsage::MethodCall(method_call.clone()),
        Expr::Block(block) => ConfUsage::Block(block.clone()),
        Expr::Call(call) => ConfUsage::Call(call.clone()),
        _ => panic!("unexpected converter"),
    }
}