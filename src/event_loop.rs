use winit::event_loop::{ActiveEventLoop as WActiveEventLoop, EventLoop as WEventLoop};
use crate::event::UserPayload;

#[napi]
pub struct EventLoop {
  pub(crate) inner: WEventLoop<()>,
}

#[napi]
impl EventLoop {
  #[napi(constructor)]
  pub fn new() -> Self {
    let event_loop = WEventLoop::new().expect("Failed to build EventLoop");
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

// #[napi(js_name = "WActiveEventLoop")]
// pub struct ActiveEventLoop {
//   pub(crate) inner: WActiveEventLoop,
// }
//
// impl From<&WActiveEventLoop> for ActiveEventLoop {
//   fn from(value: &WActiveEventLoop) -> Self {
//     Self { inner: value }
//   }
// }

/*#[napi]
impl ActiveEventLoop {
  pub fn new() -> Self {
    let event_loop = EventLoop::new().expect("Failed to build EventLoop");
    ActiveEventLoop { inner: event_loop }
  }
}*/