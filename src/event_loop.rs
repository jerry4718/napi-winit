use winit::event_loop::{
    ActiveEventLoop as OriginActiveEventLoop,
    EventLoop as OriginEventLoop,
    AsyncRequestSerial as OriginAsyncRequestSerial
};
use proc::simple_struct;
use crate::event::UserPayload;
use crate::mark_ex_into;

#[napi]
pub struct EventLoop {
    pub(crate) inner: OriginEventLoop<UserPayload>,
}

#[napi]
impl EventLoop {
    #[napi(constructor)]
    pub fn new() -> Self {
        let event_loop = OriginEventLoop::<UserPayload>::with_user_event().build().expect("Failed to build EventLoop");
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

// #[napi(js_name = "OriginActiveEventLoop")]
// pub struct ActiveEventLoop {
//     pub(crate) inner: OriginActiveEventLoop,
// }
//
// impl From<&OriginActiveEventLoop> for ActiveEventLoop {
//     fn from(value: &OriginActiveEventLoop) -> Self {
//         Self { inner: value }
//     }
// }

/*#[napi]
impl ActiveEventLoop {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().expect("Failed to build EventLoop");
        ActiveEventLoop { inner: event_loop }
    }
}*/

pub struct ActiveEventLoop {
    pub(crate) inner: OriginActiveEventLoop,
}

simple_struct!(AsyncRequestSerial);

mark_ex_into!(OriginAsyncRequestSerial, AsyncRequestSerial);