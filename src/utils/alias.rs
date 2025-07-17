use napi::bindgen_prelude::FromNapiValue;
use napi::{Status, Unknown};
use napi::threadsafe_function::ThreadsafeFunction;

pub type ThreadsafeNoCallee<T, Return = Unknown<'static>> = ThreadsafeFunction<T, Return, T, Status, false>;