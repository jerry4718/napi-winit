#[inline]
pub(crate) fn path_buf_to_string(path_buf: std::path::PathBuf) -> String {
    path_buf.to_str().unwrap().into()
}

#[inline]
pub(crate) fn option_map<T, U, Conv: FnOnce(T) -> U>(conv: Conv) -> impl FnOnce(Option<T>) -> Option<U> {
    |input: Option<T>| input.map(conv)
}

#[inline]
pub(crate) fn result_map<T, E, U, Conv: FnOnce(T) -> U>(conv: Conv) -> impl FnOnce(Result<T, E>) -> Result<U, E> {
    |input: Result<T, E>| input.map(conv)
}

#[inline]
pub(crate) fn result_map_err<T, E, EO, Conv: FnOnce(E) -> EO>(conv: Conv) -> impl FnOnce(Result<T, E>) -> Result<T, EO> {
    |input: Result<T, E>| input.map_err(conv)
}

#[inline]
pub(crate) fn option_into<T: Into<V>, V>(from: Option<T>) -> Option<V> {
    option_map(Into::into)(from)
}

#[inline]
pub(crate) fn to_option_string<T: ToString>(input: Option<T>) -> Option<String> {
    input.map(|it| it.to_string())
}

#[inline]
pub(crate) fn result_into<T: Into<V>, E, V>(input: Result<T, E>) -> Result<V, E> {
    result_map(Into::into)(input)
}

#[inline]
pub(crate) fn result_err_reason<T, E: std::fmt::Display>(input: Result<T, E>) -> napi::Result<T> {
    result_map_err(|e| crate::napi_reason!("{e}"))(input)
}

#[inline]
pub(crate) fn vec_map_into<T: Into<V>, V>(input: Vec<T>) -> Vec<V> {
    input.into_iter().map(Into::into).collect()
}

#[inline]
pub(crate) fn iterator_to_vec<T>(input: impl Iterator<Item = T>) -> Vec<T> {
    input.collect()
}

#[inline]
pub(crate) fn vec_map<T, U, Conv>(conv: Conv) -> impl FnOnce(Vec<T>) -> Vec<U>
where
    Conv: Fn(&T) -> U,
{
    |input: Vec<T>| input.iter().map(conv).collect::<Vec<U>>()
}

pub(crate) fn pipe<Mid, T, U>(conv1: impl Fn(T) -> Mid, conv2: impl Fn(Mid) -> U) -> impl Fn(T) -> U {
    move |input: T| conv2(conv1(input))
}

#[inline]
pub(crate) fn ref_clone_into<T: Clone + Into<U>, U>(input: &T) -> U {
    input.clone().into()
}

#[inline]
pub(crate) fn string_as_str(input: &String) -> &str {
    input.as_str()
}