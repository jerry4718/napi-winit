use crate::{
    conf_convert::{parse_conf_convert, ConfConvert, NormalConfConvert},
    conf_usage::{get_meta_value_as_conf_usage, quote_option_conf_usage},
    utils::{append_to_tokens, get_ident_optional, get_metas_by_attr_name, get_type_ty_or, parse_metas, separate_attr_by_name, to_case},
};
use convert_case::Case;
use macros::define_const_str;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned, ToTokens};
use syn::{parse_macro_input, Attribute, Field, Ident, ItemStruct, LitStr, Meta, Type};

pub(crate) fn proxy_wrap(attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let metas = parse_metas(attrs);

    let macro_input = parse_macro_input!(input as ItemStruct);

    let proxy_wrap = parse_proxy_wrap(&metas, &macro_input);

    proc_macro::TokenStream::from(quote! { #proxy_wrap })
}

macro_rules! map_meta_to_local {
    ($from:expr => { $($name:expr => $local:ident),* $(,)? }) => {
        let metas = ($from);
        $(let $local = $crate::utils::get_meta_by_name(metas, $name);)*
    };
}

define_const_str!(ATTR_PROXY_WRAP = proxy_wrap);
const ATTR_INCLUDES: &[&str] = &[ATTR_PROXY_WRAP];

define_const_str!(
    META_ORIGIN_TYPE = origin_type,
    META_FIELD_NAME = field_name,
    META_USE_NON_NULL = UseNonNull,
    META_USE_BOX = UseBox,
);

define_const_str!(
    META_NO_GETTER = no_getter,
    META_CONV_GET = conv_get,
    META_GET_REF = get_ref,
    META_NO_SETTER = no_setter,
    META_CONV_SET = conv_set,
);

struct ProxyWrap {
    pub input: ItemStruct,
    pub reserved_attrs: Vec<Attribute>,
    pub origin_type: Type,
    pub field_name: Option<Ident>,
    pub conf_convert: ConfConvert,
    pub no_getter: bool,
    pub no_setter: bool,
    pub use_non_null: bool,
    pub use_box: bool,
}

fn parse_proxy_wrap(metas: &Vec<Meta>, item_struct: &ItemStruct) -> ProxyWrap {
    let ItemStruct { attrs, ident, .. } = item_struct;

    let (matched, surplus) = separate_attr_by_name(attrs, ATTR_INCLUDES);
    if matches!(matched.len(), n if n > 0) {
        panic!("so many proxy_wrap");
    }

    map_meta_to_local!(&metas => {
        META_ORIGIN_TYPE => origin_type,
        META_FIELD_NAME => field_name,
        META_USE_NON_NULL => use_non_null,
        META_USE_BOX => use_box,
        META_NO_GETTER => no_getter,
        META_NO_SETTER => no_setter,
    });

    ProxyWrap {
        input: item_struct.clone(),
        reserved_attrs: surplus,
        origin_type: get_type_ty_or(&origin_type, &format_ident!("Origin{}", ident)),
        field_name: get_ident_optional(&field_name),
        conf_convert: parse_conf_convert(metas),
        no_getter: no_getter.is_some(),
        no_setter: no_setter.is_some(),
        use_non_null: use_non_null.is_some(),
        use_box: use_box.is_some(),
    }
}

impl ToTokens for ProxyWrap {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            input, reserved_attrs, origin_type, field_name,
            no_getter: root_no_getter, no_setter: root_no_setter, conf_convert,
            use_non_null, use_box,
        } = self;

        let ItemStruct { ident, vis, fields, .. } = input;
        let NormalConfConvert { skip_from_origin, skip_into_origin, skip_to_js, skip_from_js } = conf_convert.normal();

        let mut napi_metas = Vec::new();
        if skip_to_js { napi_metas.push(quote! {object_to_js = false}) }
        if skip_from_js { napi_metas.push(quote! {object_from_js = false}) }

        let wrapped_type = match use_non_null {
            true => quote! { std::ptr::NonNull<#origin_type> },
            false => quote! { #origin_type },
        };

        let wrap_body = match &field_name {
            Some(ident) => quote! { { pub(crate) #ident: #wrapped_type } },
            None => quote! { (pub(crate) #wrapped_type); },
        };

        let inner_expr = match (&field_name, use_non_null) {
            (Some(ident), false) => quote! { self.#ident },
            (None, false) => quote! { self.0 },
            (Some(ident), true) => quote! { unsafe { self.#ident.as_ref() } },
            (None, true) => quote! { unsafe { self.0.as_ref() } },
        };

        let inner_mut_expr = match (&field_name, use_non_null) {
            (Some(ident), false) => quote! { self.#ident },
            (None, false) => quote! { self.0 },
            (Some(ident), true) => quote! { unsafe { self.#ident.as_mut() } },
            (None, true) => quote! { unsafe { self.0.as_mut() } },
        };

        append_to_tokens(tokens, quote! {
            #[napi( #( #napi_metas ),* )]
            #( #reserved_attrs )*
            #vis struct #ident #wrap_body
        });

        if !fields.is_empty() {
            let fields = fields.iter().zip(0..fields.len())
                .map(|(field, fdx)| {
                    let Field { attrs, ident: origin_ident, ty, .. } = field;

                    let pat_pos = origin_ident.clone()
                        .map(|ident| quote! { #ident })
                        .unwrap_or_else(|| quote! { #fdx });

                    let ident = origin_ident.clone()
                        .unwrap_or_else(|| format_ident!("field_{}", fdx));

                    let js_name_string = to_case(quote! { #ident }.to_string(), Case::Camel);

                    let js_name = LitStr::new(&*js_name_string, ident.span());

                    let (matched, _) = separate_attr_by_name(attrs, ATTR_INCLUDES);

                    map_meta_to_local!(&get_metas_by_attr_name(&matched, ATTR_PROXY_WRAP) => {
                        META_NO_GETTER => no_getter,
                        META_CONV_GET => conv_get,
                        META_GET_REF => get_ref,
                        META_NO_SETTER => no_setter,
                        META_CONV_SET => conv_set,
                    });

                    let mut fns = TokenStream::default();

                    let no_getter = *root_no_getter || no_getter.is_some();
                    let no_setter = *root_no_setter || no_setter.is_some();

                    if !no_getter {
                        let getter = format_ident!("___get_{}", ident);
                        let use_ref = get_ref.map(|_| { quote! { ref } });
                        let conv_get = conv_get.as_ref().and_then(get_meta_value_as_conf_usage);

                        let local_ident = quote! { val };
                        let convert_code = quote_option_conf_usage(&local_ident, &conv_get);
                        append_to_tokens(&mut fns, quote_spanned! { ident.span() =>
                            #[napi(getter, js_name = #js_name)]
                            pub fn #getter (&self) -> #ty {
                                let #origin_type { #pat_pos: #use_ref #local_ident, .. } = #inner_expr;
                                #convert_code
                            }
                        });
                    }

                    if !no_setter {
                        let setter = format_ident!("___set_{}", ident);
                        let conv_set = conv_set.as_ref().and_then(get_meta_value_as_conf_usage);

                        let local_ident = quote! { val };
                        let convert_code = quote_option_conf_usage(&local_ident, &conv_set);

                        append_to_tokens(&mut fns, quote_spanned! { ident.span() =>
                            #[napi(setter, js_name = #js_name)]
                            pub fn #setter (&self, #local_ident: #ty) {
                                #inner_mut_expr.#pat_pos = #convert_code;
                            }
                        });
                    }

                    fns
                })
                .collect::<Vec<_>>();
            append_to_tokens(tokens, quote! {
                #[napi]
                impl #ident {
                    #( #fields )*
                }
            });
        }

        if !skip_from_origin {
            let from_code = match &field_name {
                Some(field) => quote! { #ident { #field: value } },
                None => quote! { #ident (value) },
            };

            append_to_tokens(tokens, quote! {
                impl From<#origin_type> for #ident {
                    fn from(value: #origin_type) -> Self {
                        #from_code
                    }
                }
            });
        }

        if !skip_into_origin {
            let into_code = match &field_name {
                Some(ident) => quote! { self.#ident },
                None => quote! { self.0 },
            };

            append_to_tokens(tokens, quote! {
                impl Into<#origin_type> for #ident {
                    fn into(self) -> #origin_type {
                        #into_code
                    }
                }
            });
        }
    }
}