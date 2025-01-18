#![deny(clippy::all)]

#![allow(unused_imports, unused_variables, dead_code)]

#[macro_use]
extern crate napi_derive;

mod dpi;
mod extra;
mod application;
mod event_loop;
mod event;
mod window;
mod cursor;
mod keyboard;
mod monitor;
mod r#macro;

#[napi]
pub fn thread_sleep(millis: f64) {
    std::thread::sleep(std::time::Duration::from_millis(millis as u64));
}

#[napi]
pub async fn tokio_sleep(millis: f64) {
    tokio::time::sleep(std::time::Duration::from_millis(millis as u64)).await;
}