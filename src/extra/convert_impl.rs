use std::path::PathBuf;
use crate::extra::convert::{ExFrom, ExInto};

impl ExFrom<PathBuf, false> for String {
    fn ex_from(from: PathBuf) -> Self {
        from.to_str().unwrap().into()
    }
}

#[cfg(test)]
mod conversion_tests {
    use std::path::PathBuf;
    use crate::extra::convert::ExFrom;

    #[test]
    pub fn test_0001() {
        let mut path_buf = PathBuf::new();
        path_buf.push("src");
        path_buf.push("lib.rs");

        let str = String::ex_from(path_buf);

        dbg!(str);
    }
}