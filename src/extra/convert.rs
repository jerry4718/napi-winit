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

pub trait ExInto<T, const DENIED: bool = false /* why denied? f**k you! */> {
    fn ex_into(self) -> T;
}

impl<T, V> ExInto<T, true> for V
where
    V: Into<T>,
{
    fn ex_into(self) -> T {
        self.into()
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
}