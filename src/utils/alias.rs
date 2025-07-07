use napi::Status;
use napi::threadsafe_function::ThreadsafeFunction;

pub type ThreadsafeNoCallee<T, R> = ThreadsafeFunction<T, R, T, Status, false>;