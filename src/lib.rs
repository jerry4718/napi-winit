#![deny(clippy::all)]
#![allow(unused_imports, unused_variables, dead_code)]

#[macro_use]
extern crate napi_derive;

use std::sync::OnceLock;

mod dpi;

mod application;
mod cursor;
mod event;
mod event_loop;
mod extra;
mod keyboard;
mod r#macro;
mod monitor;
mod utils;
mod window;
