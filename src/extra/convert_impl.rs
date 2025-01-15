use std::path::PathBuf;
use winit::keyboard::SmolStr;
use crate::extra::convert::{ExFrom, ExInto};

impl<T, V> ExFrom<Option<T>> for Option<V>
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

impl<T, V> ExInto<Option<T>> for Option<V>
where
    V: Into<T>,
{
    fn ex_into(self) -> Option<T> {
        match self {
            None => None,
            Some(from) => Some(from.into()),
        }
    }
}

impl ExFrom<PathBuf> for String {
    fn ex_from(from: PathBuf) -> Self {
        from.to_str().unwrap().into()
    }
}

impl ExFrom<SmolStr> for String {
    fn ex_from(from: SmolStr) -> Self {
        from.to_string()
    }
}

impl ExFrom<char> for String {
    fn ex_from(from: char) -> Self {
        from.to_string()
    }
}

#[cfg(test)]
mod conversion_tests {
    use std::path::PathBuf;
    use crate::extra::convert::{ExFrom, ExInto};

    #[test]
    pub fn test_0001() {
        let mut path_buf = PathBuf::new();
        path_buf.push("src");
        path_buf.push("lib.rs");

        let str = String::ex_from(path_buf);

        assert_eq!(str, "src/lib.rs".to_string());
    }

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
    fn test_ex_from_option() {
        let fr = Some(Fr { x: 8 });
        let to: Option<To> = Option::<To>::ex_from(fr);
        assert_eq!(to, Some(To { y: 8 }));
    }

    #[test]
    fn test_ex_into_option() {
        let fr = Some(Fr { x: 8 });
        let to: Option<To> = Option::<Fr>::ex_into(fr);
        assert_eq!(to, Some(To { y: 8 }));
    }
}