pub(crate) fn path_buf_to_string(path_buf: std::path::PathBuf) -> String {
    path_buf.to_str().unwrap().into()
}

pub(crate) fn option_into<T: Into<V>, V>(from: Option<T>) -> Option<V> {
    from.map(Into::into)
}

pub(crate) fn to_option_string<T: ToString>(input: Option<T>) -> Option<String> {
    input.map(|it| it.to_string())
}