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