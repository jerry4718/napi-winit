pub trait ExIntoTag<T> {}
pub trait ExOptionInto {}

pub trait ExInto<To> {
    fn ex_into(self) -> To;
}

impl<To, Fr> ExInto<To> for Fr
where
    Fr: Into<To> + ExIntoTag<To>,
{
    #[inline]
    fn ex_into(self) -> To {
        self.into()
    }
}

impl<To, Fr> ExInto<Option<To>> for Option<Fr>
where
    Fr: ExInto<To> + ExIntoTag<To>,
{
    #[inline]
    fn ex_into(self) -> Option<To> {
        match self {
            None => None,
            Some(from) => Some(from.ex_into()),
        }
    }
}

#[macro_export] macro_rules! impl_from_into {
    ($pa: path => $pb:path: $from: ident =>  { $t_from: expr }) => {
        impl $crate::extra::convert::ExInto<$pb> for $pa {
            fn ex_into(self) -> $pb {
                let $from = self;
                $t_from
            }
        }
        impl $crate::extra::convert::ExInto<Option<$pb>> for Option<$pa> {
            fn ex_into(self) -> Option<$pb> {
                match self {
                    None => None,
                    Some($from) => Some($t_from),
                }
            }
        }
    };
}

// #[macro_export] macro_rules! impl_for_auto {
//     ($($path: path), *) => {
//         $(impl $crate::extra::convert::ExFromTag for $path {})*
//     };
// }

#[macro_export] macro_rules! mark_ex_into {
    ($($path: path), *) => {
        $(impl<T> $crate::extra::convert::ExIntoTag<T> for $path where $path: Into<T> {})*
    };
    ($(($($path: path), *)),*) => {
        $(impl<T> $crate::extra::convert::ExIntoTag<T> for ($($path),*) where ($($path),*): Into<T> {})*
    }
}

mod local_impl {
    use std::path::PathBuf;
    use winit::keyboard::SmolStr;
    use crate::extra::convert::ExIntoTag;

    impl_from_into!(PathBuf => String: from => { from.to_str().unwrap().into() });
    // impl_from_into!(SmolStr => String: from => { from.to_string() });
    impl_from_into!(char => String: from => { from.to_string() });

    mark_ex_into!(String);

    mark_ex_into!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use crate::extra::convert::{ExInto, ExIntoTag, ExOptionInto};

    #[derive(Debug)]
    struct Fr {
        x: i32,
    }

    #[derive(Debug, Eq, PartialEq)]
    struct To {
        y: i32,
    }


    impl From<Fr> for To {
        fn from(value: Fr) -> Self {
            let Fr { x } = value;
            Self { y: x }
        }
    }
    impl ExIntoTag<To> for Fr {}

    #[test]
    fn test_from() {
        let fr = Fr { x: 8 };
        let to = To::from(fr);
        assert_eq!(to, To { y: 8 });
    }

    #[test]
    fn test_into() {
        let fr = Fr { x: 8 };
        let to: To = fr.ex_into();
        assert_eq!(to, To { y: 8 });
    }

    #[test]
    fn test_ex_into() {
        let fr = Fr { x: 8 };
        let to: To = fr.ex_into();
        assert_eq!(to, To { y: 8 });
    }

    #[test]
    fn test_ex_into_option() {
        let fr = Some(Fr { x: 8 });
        let to: Option<To> = Option::<Fr>::ex_into(fr);
        assert_eq!(to, Some(To { y: 8 }));
    }

    #[test]
    pub fn test_0001() {
        let mut path_buf = PathBuf::new();
        path_buf.push("src");
        path_buf.push("lib.rs");

        let str = path_buf.ex_into();

        assert_eq!(str, "src/lib.rs".to_string());
    }
}