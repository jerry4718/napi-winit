use napi::{
    Status, Unknown, bindgen_prelude::FromNapiValue, threadsafe_function::ThreadsafeFunction,
};

pub type ThreadsafeNoCallee<T, Return = Unknown<'static>> =
    ThreadsafeFunction<T, Return, T, Status, false>;
