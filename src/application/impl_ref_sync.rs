use napi::bindgen_prelude::*;
use std::ptr::NonNull;

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
    application::public::{OptionsGhostHolder, OptionsRefHolder},
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

macro_rules! wrap_event_loop {
    ($name: expr) => { ActiveEventLoop { inner_non_null: NonNull::new($name as *const _ as *mut OriginActiveEventLoop).unwrap() } };
}

macro_rules! borrow_back_callback {
    ($callback: ident as $local: ident with $env: ident from $self: ident) => {
        {
            let Self { env, options: OptionsGhostHolder { $callback: $local, .. } } = &$self;
            $local.borrow_back(env).unwrap()
        }
    };
    ($callback: ident? as $local: ident with $env: ident from $self: ident) => {
        {
            let Self { env, options: OptionsGhostHolder { $callback: Some($local), .. } } = &$self else { return; };
            $local.borrow_back(env).unwrap()
        }
    };
}

impl<'scope> ApplicationHandler<UserPayload> for OptionsRefHolder<Unknown<'scope>> {
    fn new_events(&mut self, event_loop: &OriginActiveEventLoop, cause: OriginStartCause) {
        let callback = borrow_back_callback!(on_new_events? as callback with env from self);
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop), StartCause::from(cause))));
        handle_res!(result);
    }

    fn resumed(&mut self, event_loop: &OriginActiveEventLoop) {
        let callback = borrow_back_callback!(on_resumed as callback with env from self);
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn user_event(&mut self, event_loop: &OriginActiveEventLoop, event: UserPayload) {
        let callback = borrow_back_callback!(on_user_event? as callback with env from self);
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop), event)));
        handle_res!(result);
    }

    fn window_event(&mut self, event_loop: &OriginActiveEventLoop, window_id: OriginWindowId, event: OriginWindowEvent) {
        let callback = borrow_back_callback!(on_window_event as callback with env from self);
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop), WindowId::from(window_id), WindowEvent::from(event))));
        handle_res!(result);
    }

    fn device_event(&mut self, event_loop: &OriginActiveEventLoop, device_id: OriginDeviceId, event: OriginDeviceEvent) {
        let callback = borrow_back_callback!(on_device_event? as callback with env from self);
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop), DeviceId::from(device_id), DeviceEvent::from(event))));
        handle_res!(result);
    }

    fn about_to_wait(&mut self, event_loop: &OriginActiveEventLoop) {
        let callback = borrow_back_callback!(on_about_to_wait? as callback with env from self);
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn suspended(&mut self, event_loop: &OriginActiveEventLoop) {
        let callback = borrow_back_callback!(on_suspended? as callback with env from self);
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn exiting(&mut self, event_loop: &OriginActiveEventLoop) {
        let callback = borrow_back_callback!(on_exiting? as callback with env from self);
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }

    fn memory_warning(&mut self, event_loop: &OriginActiveEventLoop) {
        let callback = borrow_back_callback!(on_memory_warning? as callback with env from self);
        let result = callback.call(FnArgs::from((wrap_event_loop!(event_loop),)));
        handle_res!(result);
    }
}