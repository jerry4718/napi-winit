#![deny(clippy::all)]
#![allow(unused_imports, unused_variables, dead_code)]

// #[cfg(not(target_env = "msvc"))]
// use jemallocator::Jemalloc;

// #[cfg(not(target_env = "msvc"))]
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

#[macro_use]
extern crate napi_derive;

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