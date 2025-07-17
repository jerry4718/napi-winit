use napi::bindgen_prelude::*;
use std::ptr::NonNull;
use napi::threadsafe_function::ThreadsafeFunctionCallMode;

use winit::{
    application::ApplicationHandler,
    event::{
        DeviceEvent as OriginDeviceEvent,
        DeviceId as OriginDeviceId,
        StartCause as OriginStartCause,
        WindowEvent as OriginWindowEvent,
    },
    event_loop::ActiveEventLoop as OriginActiveEventLoop,
    window::WindowId as OriginWindowId,
};

use crate::{
    application::public::{
        OptionsSafeHolder
    },
    event::{
        DeviceEvent,
        DeviceId,
        StartCause,
        UserPayload,
        WindowEvent,
    },
    event_loop::ActiveEventLoop,
    handle_res,
    window::WindowId,
};

macro_rules! call {
    ($self: ident, $func: ident, $($args: expr), +) => {
        let Self { $func: $func, .. } = &$self;
        call!(call $func $(, $args)+);
    };
    ($self: ident, $func: ident?, $($args: expr), +) => {
        let Self { $func: Some($func), .. } = &$self else { return; };
        call!(call $func $(, $args)+);
    };
    (call $fx: ident, $($args: expr), +) => {
        let result = $fx.call(FnArgs::from(($(From::from($args), )+)), ThreadsafeFunctionCallMode::NonBlocking);
        if Status::Ok != result { dbg!(result); };
    }
}

impl ApplicationHandler<UserPayload> for OptionsSafeHolder<Option<Promise<()>>> {
    fn new_events(&mut self, event_loop: &OriginActiveEventLoop, cause: OriginStartCause) {
        call!(self, on_new_events?, event_loop, cause);
    }

    fn resumed(&mut self, event_loop: &OriginActiveEventLoop) {
        call!(self, on_resumed, event_loop);
    }

    fn user_event(&mut self, event_loop: &OriginActiveEventLoop, event: UserPayload) {
        call!(self, on_user_event?, event_loop, event);
    }

    fn window_event(&mut self, event_loop: &OriginActiveEventLoop, window_id: OriginWindowId, event: OriginWindowEvent) {
        call!(self, on_window_event, event_loop, window_id, event);
    }

    fn device_event(&mut self, event_loop: &OriginActiveEventLoop, device_id: OriginDeviceId, event: OriginDeviceEvent) {
        call!(self, on_device_event?, event_loop, device_id, event);
    }

    fn about_to_wait(&mut self, event_loop: &OriginActiveEventLoop) {
        call!(self, on_about_to_wait?, event_loop);
    }

    fn suspended(&mut self, event_loop: &OriginActiveEventLoop) {
        call!(self, on_suspended?, event_loop);
    }

    fn exiting(&mut self, event_loop: &OriginActiveEventLoop) {
        call!(self, on_exiting?, event_loop);
    }

    fn memory_warning(&mut self, event_loop: &OriginActiveEventLoop) {
        call!(self, on_memory_warning?, event_loop);
    }
}