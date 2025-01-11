#![deny(clippy::all)]

#![allow(unused_imports, unused_variables, dead_code)]

#[macro_use]
extern crate napi_derive;

mod dpi;
mod js;
mod application;
mod event_loop;
mod event;
mod window;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
