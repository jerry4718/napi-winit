use syn::Meta;
use macros::{define_const_str, map_meta_to_local};

pub(crate) struct ConfConvert {
    pub skip_from_origin: bool,
    pub skip_into_origin: bool,
    pub skip_to_js: bool,
    pub skip_from_js: bool,
    pub skip_forward: bool,
    pub skip_backward: bool,
}

pub(crate) struct NormalConfConvert {
    pub skip_from_origin: bool,
    pub skip_into_origin: bool,
    pub skip_to_js: bool,
    pub skip_from_js: bool,
}

impl ConfConvert {
    pub fn normal(&self) -> NormalConfConvert {
        let Self {
            skip_from_origin, skip_into_origin,
            skip_to_js, skip_from_js,
            skip_forward, skip_backward,
        } = self;
        NormalConfConvert {
            skip_from_origin: *skip_from_origin || *skip_forward,
            skip_into_origin: *skip_into_origin || *skip_backward,
            skip_to_js: *skip_to_js || *skip_forward,
            skip_from_js: *skip_from_js || *skip_backward,
        }
    }
}

define_const_str!(
    META_SKIP_FROM_ORIGIN = skip_from_origin,
    META_SKIP_INTO_ORIGIN = skip_into_origin,
    META_SKIP_TO_JS = skip_to_js,
    META_SKIP_FROM_JS = skip_from_js,
    META_SKIP_FORWARD = skip_forward,
    META_SKIP_BACKWARD = skip_backward,
);

pub(crate) fn parse_conf_convert(metas: &Vec<Meta>) -> ConfConvert {
    map_meta_to_local!(&metas => {
        META_SKIP_FROM_ORIGIN => skip_from_origin,
        META_SKIP_INTO_ORIGIN => skip_into_origin,
        META_SKIP_TO_JS => skip_to_js,
        META_SKIP_FROM_JS => skip_from_js,
        META_SKIP_FORWARD => skip_forward,
        META_SKIP_BACKWARD => skip_backward,
    });

    ConfConvert {
        skip_from_origin: skip_from_origin.is_some(),
        skip_into_origin: skip_into_origin.is_some(),
        skip_to_js: skip_to_js.is_some(),
        skip_from_js: skip_from_js.is_some(),
        skip_forward: skip_forward.is_some(),
        skip_backward: skip_backward.is_some(),
    }
}