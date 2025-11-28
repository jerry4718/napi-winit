use crate::utils::get_meta_by_name;
use syn::Meta;

pub(crate) trait MetaGetter<T> {
    type Output;
    fn get(&self, metas: &Vec<Meta>) -> Self::Output;
}

macro_rules! seat_tt {
    ($name: ident => $($tt: tt)*) => { $($tt)* };
}

macro_rules! impl_meta_getter {
    ($($name:ident),*) => {
        impl MetaGetter<Self> for ($(seat_tt!( $name => &str )),*,) {
            type Output = ($(seat_tt!( $name => Option<Meta> )),*,);

            fn get(&self, metas: &Vec<Meta>) -> Self::Output {
                let ($($name),*,) = self;
                ($(get_meta_by_name(metas, $name)),*,)
            }
        }
    };
}

impl_meta_getter!(name0);
impl_meta_getter!(name0, name1);
impl_meta_getter!(name0, name1, name2);
impl_meta_getter!(name0, name1, name2, name3);
impl_meta_getter!(name0, name1, name2, name3, name4);
impl_meta_getter!(name0, name1, name2, name3, name4, name5);
impl_meta_getter!(name0, name1, name2, name3, name4, name5, name6);
impl_meta_getter!(name0, name1, name2, name3, name4, name5, name6, name7);
impl_meta_getter!(name0, name1, name2, name3, name4, name5, name6, name7, name8);
impl_meta_getter!(name0, name1, name2, name3, name4, name5, name6, name7, name8, name9);

#[inline]
pub(crate) fn get_meta_by_names<T: MetaGetter<T>>(metas: &Vec<Meta>, names: T) -> <T as MetaGetter<T>>::Output {
    names.get(metas)
}
