pub trait ExFrom<T, const DENIED: bool = false /* why denied? f**k you! */> {
    fn ex_from(from: T) -> Self;
}

impl<T, V> ExFrom<T, true> for V
where
    V: From<T>,
{
    fn ex_from(from: T) -> V {
        V::from(from)
    }
}

impl<T, V> ExFrom<Option<T>, false> for Option<V>
where
    V: From<T>,
{
    fn ex_from(from: Option<T>) -> Option<V> {
        match from {
            None => None,
            Some(from) => Some(V::ex_from(from)),
        }
    }
}

pub trait ExInto<T, const DENIED: bool = false /* why denied? f**k you! */> {
    fn ex_into(self) -> T;
}

impl<T, V> ExInto<T, true> for V
where
    T: ExFrom<V, true>,
{
    fn ex_into(self) -> T {
        T::ex_from(self)
    }
}

impl<T, V> ExInto<T, false> for V
where
    T: ExFrom<V, false>,
{
    fn ex_into(self) -> T {
        T::ex_from(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::extra::convert::{ExFrom, ExInto};

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

    #[test]
    fn test_from() {
        let fr = Fr { x: 8 };
        let to = To::from(fr);
        assert_eq!(to, To { y: 8 });
    }

    #[test]
    fn test_ex_from() {
        let fr = Fr { x: 8 };
        let to = To::ex_from(fr);
        assert_eq!(to, To { y: 8 });
    }

    #[test]
    fn test_ex_from_option() {
        let fr = Some(Fr { x: 8 });
        let to: Option<To> = Option::<To>::ex_from(fr);
        assert_eq!(to, Some(To { y: 8 }));
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
}