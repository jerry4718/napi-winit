#![deny(clippy::all)]

#![allow(unused_imports, unused_variables)]

mod dpi;
mod application;
mod event_loop;
mod event;
mod window;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}
