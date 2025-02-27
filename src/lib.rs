#![deny(clippy::all)]

#![allow(unused_imports, unused_variables, dead_code)]

#[macro_use]
extern crate napi_derive;

use std::ptr::NonNull;
use napi::bindgen_prelude::*;
use napi::{JsUndefined, JsUnknown};
use once_cell::sync::Lazy;

mod dpi;
mod extra;
mod event_loop;
mod event;
mod window;
mod cursor;
mod keyboard;
mod monitor;
mod r#macro;
mod async_helper;
mod application;

pub static THREAD_POOL: Lazy<threadpool::ThreadPool> = Lazy::new(|| {
    threadpool::ThreadPool::default()
});