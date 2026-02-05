#![deny(clippy::all)]
#![allow(unused_imports, unused_variables, dead_code)]

#[macro_use]
extern crate napi_derive;

use std::sync::OnceLock;
use crate::event::UserPayload;

mod dpi;

mod extra;
mod event_loop;
mod event;
mod window;
mod cursor;
mod keyboard;
mod monitor;
mod r#macro;
mod application;
mod utils;

pub static THREAD_POOL: OnceLock<threadpool::ThreadPool> = OnceLock::new();

fn get_thread_pool() -> &'static threadpool::ThreadPool {
    THREAD_POOL.get_or_init(|| threadpool::ThreadPool::default())
}