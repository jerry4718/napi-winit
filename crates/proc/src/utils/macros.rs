macro_rules! map_meta_to_local {
    ($from:expr => { $($name:expr => $local:ident),* $(,)? }) => {
        let metas = ($from);
        $(let $local = $crate::utils::get_meta_by_name(metas, $name);)*
    };
}

macro_rules! define_const_str {
    ($($name:ident = $inner:ident),* $(,)?) => {
        $(const $name: &str = stringify!($inner);)*
    };
}

pub(crate) use define_const_str;
pub(crate) use map_meta_to_local;
