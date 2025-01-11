use winit::event_loop::{ActiveEventLoop, EventLoop};
use crate::event::UserPayload;

#[napi(js_name = "EventLoop")]
pub struct JsEventLoop {
  pub(crate) inner: EventLoop<()>,
}

#[napi]
impl JsEventLoop {
  #[napi(constructor)]
  pub fn new() -> Self {
    let event_loop = EventLoop::new().expect("Failed to build EventLoop");
    Self { inner: event_loop }
  }
  // with_user_event
  // run
  // run_app
  // create_proxy
  // owned_display_handle
  // listen_device_events
  // set_control_flow
  // create_window
  // create_custom_cursor
}

// #[napi(js_name = "ActiveEventLoop")]
// pub struct JsActiveEventLoop {
//   pub(crate) inner: ActiveEventLoop,
// }
//
// impl From<&ActiveEventLoop> for JsActiveEventLoop {
//   fn from(value: &ActiveEventLoop) -> Self {
//     Self { inner: value }
//   }
// }

/*#[napi]
impl JsActiveEventLoop {
  pub fn new() -> Self {
    let event_loop = EventLoop::new().expect("Failed to build EventLoop");
    JsActiveEventLoop { inner: event_loop }
  }
}*/